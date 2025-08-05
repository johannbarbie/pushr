#!/usr/bin/env python3
"""
Extended comparison testing for Push3 implementations
Tests edge cases, complex programs, and stress scenarios
"""

import json
import subprocess
import sys
import os
import time
from typing import Dict, Any, List, Tuple
import traceback

class ExtendedComparison:
    def __init__(self):
        self.results = {
            "edge_cases": [],
            "complex_sequences": [],
            "type_interactions": [],
            "computation_patterns": [],
            "stress_tests": [],
            "summary": {}
        }
        self.timeout = 5  # seconds per test
        
    def run_pushr(self, program: str) -> Dict[str, Any]:
        """Run a Push program using the Rust implementation"""
        try:
            result = subprocess.run(
                ["cargo", "run", "--quiet", "--", program],
                capture_output=True,
                text=True,
                timeout=self.timeout
            )
            
            if result.returncode != 0:
                return {"error": f"Execution failed: {result.stderr}"}
            
            # Parse the output
            lines = result.stdout.strip().split('\n')
            state = {"integer": [], "float": [], "boolean": []}
            
            in_final_state = False
            for line in lines:
                if "=== FINAL STATE ===" in line:
                    in_final_state = True
                elif in_final_state:
                    if "Integer stack:" in line:
                        stack_str = line.split("Integer stack:")[1].strip()
                        if stack_str != "[]":
                            state["integer"] = eval(stack_str)
                    elif "Float stack:" in line:
                        stack_str = line.split("Float stack:")[1].strip()
                        if stack_str != "[]":
                            state["float"] = eval(stack_str)
                    elif "Boolean stack:" in line:
                        stack_str = line.split("Boolean stack:")[1].strip()
                        if stack_str != "[]":
                            # Parse boolean values properly
                            stack_str = stack_str.replace("true", "True").replace("false", "False")
                            state["boolean"] = eval(stack_str)
            
            return state
            
        except subprocess.TimeoutExpired:
            return {"error": "Execution timeout"}
        except Exception as e:
            return {"error": str(e)}
    
    def run_clojush(self, program: str) -> Dict[str, Any]:
        """Run a Push program using the Clojure implementation"""
        try:
            # Convert program format
            clojush_program = self.convert_to_clojush_format(program)
            
            result = subprocess.run(
                ["clojure", "-M", "-m", "test-runner", clojush_program],
                capture_output=True,
                text=True,
                timeout=self.timeout
            )
            
            if result.returncode != 0:
                return {"error": f"Execution failed: {result.stderr}"}
            
            # Filter out the JSON from stdout (ignore warnings)
            for line in result.stdout.strip().split('\n'):
                if line.startswith('{'):
                    state = json.loads(line)
                    # Clean up empty stacks
                    return {
                        "integer": state.get("integer", []),
                        "float": state.get("float", []),
                        "boolean": state.get("boolean", [])
                    }
            
            return {"error": "No JSON output found"}
            
        except subprocess.TimeoutExpired:
            return {"error": "Execution timeout"}
        except Exception as e:
            return {"error": str(e)}
    
    def convert_to_clojush_format(self, program: str) -> str:
        """Convert Pushr program format to Clojush format"""
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
            "FLOAT.SWAP": "float_swap",
            "FLOAT.ROT": "float_rot",
            "FLOAT.POP": "float_pop",
            "BOOLEAN.AND": "boolean_and",
            "BOOLEAN.OR": "boolean_or",
            "BOOLEAN.NOT": "boolean_not",
            "BOOLEAN.DUP": "boolean_dup",
            "BOOLEAN.SWAP": "boolean_swap",
            "BOOLEAN.ROT": "boolean_rot",
            "BOOLEAN.POP": "boolean_pop",
            "TRUE": "true",
            "FALSE": "false"
        }
        
        result = program
        for pushr_name, clojush_name in replacements.items():
            result = result.replace(pushr_name, clojush_name)
        
        return result
    
    def compare_states(self, pushr: Dict, clojush: Dict, expected: Dict = None) -> Tuple[bool, List[str]]:
        """Compare states and return (match, differences)"""
        differences = []
        
        # Handle error cases
        if "error" in pushr:
            differences.append(f"Pushr error: {pushr['error']}")
            return False, differences
        if "error" in clojush:
            differences.append(f"Clojush error: {clojush['error']}")
            return False, differences
        
        # Compare each stack type
        for stack_type in ["integer", "float", "boolean"]:
            p = pushr.get(stack_type, [])
            c = clojush.get(stack_type, [])
            
            if p != c:
                differences.append(f"{stack_type} stack: Pushr={p}, Clojush={c}")
                
            # Also check against expected if provided
            if expected and stack_type in expected:
                e = expected[stack_type]
                if p != e:
                    differences.append(f"{stack_type} stack: Pushr={p}, Expected={e}")
                if c != e:
                    differences.append(f"{stack_type} stack: Clojush={c}, Expected={e}")
        
        return len(differences) == 0, differences
    
    def run_test(self, test: Dict, category: str) -> Dict:
        """Run a single test and return results"""
        name = test['name']
        program = test['program']
        expected = test.get('expected', {})
        description = test.get('description', '')
        
        print(f"\n{name}: {description}")
        print(f"  Program: {program}")
        
        start_time = time.time()
        pushr_result = self.run_pushr(program)
        pushr_time = time.time() - start_time
        
        start_time = time.time()
        clojush_result = self.run_clojush(program)
        clojush_time = time.time() - start_time
        
        match, differences = self.compare_states(pushr_result, clojush_result, expected)
        
        result = {
            "name": name,
            "program": program,
            "description": description,
            "match": match,
            "pushr_time": pushr_time,
            "clojush_time": clojush_time
        }
        
        if match:
            print("  ✓ Results match")
            result["status"] = "pass"
            if expected:
                # Verify against expected
                match_expected = (pushr_result.get("integer", []) == expected.get("integer", []) and
                                pushr_result.get("float", []) == expected.get("float", []) and
                                pushr_result.get("boolean", []) == expected.get("boolean", []))
                if not match_expected:
                    print(f"  ⚠ Warning: Doesn't match expected result")
                    result["expected_mismatch"] = True
        else:
            print("  ✗ Results differ:")
            for diff in differences:
                print(f"    {diff}")
            result["status"] = "fail"
            result["differences"] = differences
        
        # Store actual results
        result["pushr_result"] = pushr_result
        result["clojush_result"] = clojush_result
        
        return result
    
    def run_category(self, category_name: str, tests: List[Dict]):
        """Run all tests in a category"""
        print(f"\n{'='*60}")
        print(f"CATEGORY: {category_name}")
        print(f"{'='*60}")
        
        category_results = []
        for test in tests:
            try:
                result = self.run_test(test, category_name)
                category_results.append(result)
            except Exception as e:
                print(f"  Error running test: {e}")
                traceback.print_exc()
                category_results.append({
                    "name": test['name'],
                    "status": "error",
                    "error": str(e)
                })
        
        self.results[category_name] = category_results
    
    def run_all_tests(self, test_file: str):
        """Run all tests from a test file"""
        print(f"Loading tests from {test_file}")
        
        with open(test_file, 'r') as f:
            test_suite = json.load(f)
        
        for category_name, tests in test_suite.items():
            self.run_category(category_name, tests)
        
        self.generate_summary()
        self.save_results()
    
    def generate_summary(self):
        """Generate summary statistics"""
        total = 0
        passed = 0
        failed = 0
        errors = 0
        
        category_stats = {}
        
        for category, results in self.results.items():
            if category == "summary":
                continue
                
            cat_total = len(results)
            cat_passed = sum(1 for r in results if r.get("status") == "pass")
            cat_failed = sum(1 for r in results if r.get("status") == "fail")
            cat_errors = sum(1 for r in results if r.get("status") == "error")
            
            category_stats[category] = {
                "total": cat_total,
                "passed": cat_passed,
                "failed": cat_failed,
                "errors": cat_errors,
                "pass_rate": cat_passed / cat_total * 100 if cat_total > 0 else 0
            }
            
            total += cat_total
            passed += cat_passed
            failed += cat_failed
            errors += cat_errors
        
        self.results["summary"] = {
            "total_tests": total,
            "passed": passed,
            "failed": failed,
            "errors": errors,
            "overall_pass_rate": passed / total * 100 if total > 0 else 0,
            "category_stats": category_stats
        }
        
        # Print summary
        print(f"\n{'='*60}")
        print("OVERALL SUMMARY")
        print(f"{'='*60}")
        print(f"Total tests: {total}")
        print(f"Passed: {passed} ({passed/total*100:.1f}%)")
        print(f"Failed: {failed}")
        print(f"Errors: {errors}")
        print(f"\nBy category:")
        for cat, stats in category_stats.items():
            print(f"  {cat}: {stats['passed']}/{stats['total']} ({stats['pass_rate']:.1f}%)")
    
    def save_results(self):
        """Save detailed results to file"""
        filename = "extended_test_results.json"
        with open(filename, "w") as f:
            json.dump(self.results, f, indent=2)
        print(f"\nDetailed results saved to {filename}")

def main():
    if not os.path.exists("Cargo.toml"):
        print("Error: Must run from the Pushr project root directory")
        sys.exit(1)
    
    comparison = ExtendedComparison()
    
    # Run extended tests if available
    if os.path.exists("extended_test_suite.json"):
        comparison.run_all_tests("extended_test_suite.json")
    else:
        print("Error: extended_test_suite.json not found")
        sys.exit(1)

if __name__ == "__main__":
    main()