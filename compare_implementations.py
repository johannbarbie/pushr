#!/usr/bin/env python3
"""
Compare Push3 implementations between Pushr (Rust) and Clojush (Clojure)
"""

import json
import subprocess
import sys
from typing import Dict, Any, List
import os

class ImplementationComparison:
    def __init__(self):
        self.test_results = []
        
    def run_pushr(self, program: str) -> Dict[str, Any]:
        """Run a Push program using the Rust implementation"""
        try:
            # Run cargo run with the program
            result = subprocess.run(
                ["cargo", "run", "--", program],
                capture_output=True,
                text=True,
                cwd="."
            )
            
            if result.returncode != 0:
                return {"error": f"Execution failed: {result.stderr}"}
            
            # Parse the output to extract stack states
            output_lines = result.stdout.strip().split('\n')
            state = {}
            
            for line in output_lines:
                if "Integer stack:" in line:
                    stack_str = line.split("Integer stack:")[1].strip()
                    if stack_str and stack_str != "[]":
                        # Parse the stack values
                        values = stack_str.strip("[]").split(", ")
                        state["integer"] = [int(v) for v in values if v]
                    else:
                        state["integer"] = []
                elif "Float stack:" in line:
                    stack_str = line.split("Float stack:")[1].strip()
                    if stack_str and stack_str != "[]":
                        values = stack_str.strip("[]").split(", ")
                        state["float"] = [float(v) for v in values if v]
                    else:
                        state["float"] = []
                elif "Boolean stack:" in line:
                    stack_str = line.split("Boolean stack:")[1].strip()
                    if stack_str and stack_str != "[]":
                        values = stack_str.strip("[]").split(", ")
                        state["boolean"] = [v.lower() == "true" for v in values if v]
                    else:
                        state["boolean"] = []
            
            return state
            
        except Exception as e:
            return {"error": str(e)}
    
    def run_clojush(self, program: str) -> Dict[str, Any]:
        """Run a Push program using the Clojure implementation"""
        try:
            # Convert program format from Pushr to Clojush
            clojush_program = self.convert_to_clojush_format(program)
            
            # Run the Clojure test runner
            result = subprocess.run(
                ["clojure", "-M", "-m", "test-runner", clojush_program],
                capture_output=True,
                text=True
            )
            
            if result.returncode != 0:
                return {"error": f"Execution failed: {result.stderr}"}
            
            # Parse JSON output
            return json.loads(result.stdout.strip())
            
        except Exception as e:
            return {"error": str(e)}
    
    def convert_to_clojush_format(self, program: str) -> str:
        """Convert Pushr program format to Clojush format"""
        # Replace uppercase instruction names with lowercase versions
        replacements = {
            "INTEGER.+": "integer_add",
            "INTEGER.-": "integer_sub",
            "INTEGER.*": "integer_mult",
            "INTEGER./": "integer_div",
            "INTEGER.%": "integer_mod",
            "INTEGER.DUP": "integer_dup",
            "INTEGER.SWAP": "integer_swap",
            "INTEGER.ROT": "integer_rot",
            "INTEGER.POP": "integer_pop",
            "INTEGER.=": "integer_eq",
            "INTEGER.<": "integer_lt",
            "INTEGER.>": "integer_gt",
            "FLOAT.+": "float_add",
            "FLOAT.-": "float_sub",
            "FLOAT.*": "float_mult",
            "FLOAT./": "float_div",
            "FLOAT.DUP": "float_dup",
            "BOOLEAN.AND": "boolean_and",
            "BOOLEAN.OR": "boolean_or",
            "BOOLEAN.NOT": "boolean_not",
            "TRUE": "true",
            "FALSE": "false"
        }
        
        result = program
        for pushr_name, clojush_name in replacements.items():
            result = result.replace(pushr_name, clojush_name)
        
        return result
    
    def compare_states(self, pushr_state: Dict, clojush_state: Dict, expected: Dict) -> Dict:
        """Compare the states from both implementations"""
        comparison = {
            "match": True,
            "differences": []
        }
        
        # Check each stack type
        for stack_type in ["integer", "float", "boolean"]:
            pushr_stack = pushr_state.get(stack_type, [])
            clojush_stack = clojush_state.get(stack_type, [])
            expected_stack = expected.get(stack_type, [])
            
            if pushr_stack != clojush_stack:
                comparison["match"] = False
                comparison["differences"].append({
                    "stack": stack_type,
                    "pushr": pushr_stack,
                    "clojush": clojush_stack,
                    "expected": expected_stack
                })
        
        return comparison
    
    def run_test(self, test: Dict) -> Dict:
        """Run a single test on both implementations"""
        print(f"\nRunning test: {test['name']}")
        print(f"Program: {test['program']}")
        
        # Run on both implementations
        pushr_result = self.run_pushr(test['program'])
        clojush_result = self.run_clojush(test['program'])
        
        # Handle errors
        if "error" in pushr_result:
            print(f"  Pushr error: {pushr_result['error']}")
            return {"name": test['name'], "status": "pushr_error", "error": pushr_result['error']}
        
        if "error" in clojush_result:
            print(f"  Clojush error: {clojush_result['error']}")
            return {"name": test['name'], "status": "clojush_error", "error": clojush_result['error']}
        
        # Compare results
        comparison = self.compare_states(pushr_result, clojush_result, test.get('expected', {}))
        
        if comparison["match"]:
            print("  ✓ Results match!")
            return {"name": test['name'], "status": "pass"}
        else:
            print("  ✗ Results differ:")
            for diff in comparison["differences"]:
                print(f"    {diff['stack']} stack:")
                print(f"      Pushr:   {diff['pushr']}")
                print(f"      Clojush: {diff['clojush']}")
                print(f"      Expected: {diff['expected']}")
            return {"name": test['name'], "status": "fail", "differences": comparison["differences"]}
    
    def run_test_suite(self, suite_file: str):
        """Run all tests from a test suite file"""
        with open(suite_file, 'r') as f:
            test_suite = json.load(f)
        
        total_tests = 0
        passed_tests = 0
        failed_tests = 0
        error_tests = 0
        
        for category, tests in test_suite.items():
            print(f"\n{'='*60}")
            print(f"Category: {category}")
            print(f"{'='*60}")
            
            for test in tests:
                result = self.run_test(test)
                total_tests += 1
                
                if result["status"] == "pass":
                    passed_tests += 1
                elif result["status"] == "fail":
                    failed_tests += 1
                else:
                    error_tests += 1
                
                self.test_results.append(result)
        
        # Print summary
        print(f"\n{'='*60}")
        print("SUMMARY")
        print(f"{'='*60}")
        print(f"Total tests: {total_tests}")
        print(f"Passed: {passed_tests}")
        print(f"Failed: {failed_tests}")
        print(f"Errors: {error_tests}")
        print(f"Success rate: {passed_tests/total_tests*100:.1f}%")
        
        # Save detailed results
        with open("test_results.json", "w") as f:
            json.dump(self.test_results, f, indent=2)
        print("\nDetailed results saved to test_results.json")

def main():
    # Check if we're in the right directory
    if not os.path.exists("Cargo.toml"):
        print("Error: Must run from the Pushr project root directory")
        sys.exit(1)
    
    if not os.path.exists("Clojush"):
        print("Error: Clojush directory not found. Please clone it first.")
        sys.exit(1)
    
    comparison = ImplementationComparison()
    comparison.run_test_suite("test_suite.json")

if __name__ == "__main__":
    main()