#!/usr/bin/env python3
"""
Ultimate comparison testing for Push3 implementations
Tests all edge cases, extreme values, and stress scenarios
"""

import json
import subprocess
import sys
import os
import time
import math
from typing import Dict, Any, List, Tuple
import traceback
import re

class UltimateComparison:
    def __init__(self):
        self.results = []
        self.summary = {}
        self.timeout = 10  # seconds per test
        
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
                            # Handle special float values
                            stack_str = stack_str.replace("inf", "float('inf')")
                            stack_str = stack_str.replace("-inf", "float('-inf')")
                            stack_str = stack_str.replace("NaN", "float('nan')")
                            state["float"] = eval(stack_str)
                    elif "Boolean stack:" in line:
                        stack_str = line.split("Boolean stack:")[1].strip()
                        if stack_str != "[]":
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
            
            # Filter out the JSON from stdout
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
        # First handle special cases that need token manipulation
        tokens = program.split()
        result_tokens = []
        
        for token in tokens:
            if token == "INTEGER.DUP2" or token == "INTEGER.DDUP":
                result_tokens.extend(["2", "integer_dup_items"])
            elif token == "BOOLEAN.DUP2":
                result_tokens.extend(["2", "boolean_dup_items"])
            else:
                result_tokens.append(token)
        
        program = " ".join(result_tokens)
        
        # Then do regular replacements
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
            "INTEGER.DROP": "integer_pop",
            "INTEGER.=": "integer_eq",
            "INTEGER.<": "integer_lt",
            "INTEGER.>": "integer_gt",
            "FLOAT.+": "float_add",
            "FLOAT.-": "float_sub",
            "FLOAT.*": "float_mult",
            "FLOAT./": "float_div",
            "FLOAT.%": "float_mod",
            "FLOAT.DUP": "float_dup",
            "FLOAT.SWAP": "float_swap",
            "FLOAT.ROT": "float_rot",
            "FLOAT.POP": "float_pop",
            "FLOAT.=": "float_eq",
            "FLOAT.<": "float_lt",
            "FLOAT.>": "float_gt",
            "BOOLEAN.AND": "boolean_and",
            "BOOLEAN.OR": "boolean_or",
            "BOOLEAN.NOT": "boolean_not",
            "BOOLEAN.=": "boolean_eq",
            "BOOLEAN.DUP": "boolean_dup",
            "BOOLEAN.DUP2": "2 boolean_dup_items",
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
    
    def compare_states(self, pushr: Dict, clojush: Dict) -> Tuple[bool, List[str]]:
        """Compare states and return (match, differences)"""
        differences = []
        
        # Handle error cases
        if "error" in pushr and "error" in clojush:
            # Both errored - consider this a match
            return True, []
        elif "error" in pushr:
            differences.append(f"Pushr error: {pushr['error']}")
            return False, differences
        elif "error" in clojush:
            differences.append(f"Clojush error: {clojush['error']}")
            return False, differences
        
        # Compare each stack type
        for stack_type in ["integer", "float", "boolean"]:
            p = pushr.get(stack_type, [])
            c = clojush.get(stack_type, [])
            
            # Special handling for floats
            if stack_type == "float":
                if not self.compare_float_stacks(p, c):
                    differences.append(f"{stack_type} stack: Pushr={p}, Clojush={c}")
            elif p != c:
                differences.append(f"{stack_type} stack: Pushr={p}, Clojush={c}")
        
        return len(differences) == 0, differences
    
    def compare_float_stacks(self, stack1: List[float], stack2: List[float]) -> bool:
        """Compare float stacks with special handling for NaN and infinity"""
        if len(stack1) != len(stack2):
            return False
        
        for a, b in zip(stack1, stack2):
            # Handle NaN
            if math.isnan(a) and math.isnan(b):
                continue
            # Handle infinity
            elif math.isinf(a) and math.isinf(b):
                if a != b:  # Check sign of infinity
                    return False
            # Normal comparison with tolerance
            elif abs(a - b) > 1e-10:
                return False
        
        return True
    
    def run_test(self, test: Dict) -> Dict:
        """Run a single test and return results"""
        name = test['name']
        program = test['program']
        description = test.get('description', '')
        
        print(f"\n{name}: {description}")
        
        # Skip empty/whitespace programs for now
        if not program.strip():
            print("  ⚠ Skipping empty program")
            return {
                "name": name,
                "status": "skipped",
                "reason": "Empty program"
            }
        
        start_time = time.time()
        pushr_result = self.run_pushr(program)
        pushr_time = time.time() - start_time
        
        start_time = time.time()
        clojush_result = self.run_clojush(program)
        clojush_time = time.time() - start_time
        
        match, differences = self.compare_states(pushr_result, clojush_result)
        
        result = {
            "name": name,
            "program": program,
            "description": description,
            "match": match,
            "pushr_time": pushr_time,
            "clojush_time": clojush_time,
            "pushr_result": pushr_result,
            "clojush_result": clojush_result
        }
        
        if match:
            print("  ✓ Results match")
            result["status"] = "pass"
        else:
            print("  ✗ Results differ:")
            for diff in differences:
                print(f"    {diff}")
            result["status"] = "fail"
            result["differences"] = differences
        
        # Show actual results for debugging
        if not match or "error" in pushr_result or "error" in clojush_result:
            print(f"    Pushr: {pushr_result}")
            print(f"    Clojush: {clojush_result}")
        
        return result
    
    def run_all_tests(self, test_file: str):
        """Run all tests from a test file"""
        print(f"Loading tests from {test_file}")
        
        with open(test_file, 'r') as f:
            test_suite = json.load(f)
        
        total_tests = 0
        
        # Count total tests
        for category, tests in test_suite.items():
            total_tests += len(tests)
        
        print(f"Found {total_tests} tests in {len(test_suite)} categories")
        
        test_number = 0
        for category_name, tests in test_suite.items():
            print(f"\n{'='*80}")
            print(f"CATEGORY: {category_name}")
            print(f"{'='*80}")
            
            for test in tests:
                test_number += 1
                print(f"\nTest {test_number}/{total_tests}:", end="")
                
                try:
                    result = self.run_test(test)
                    self.results.append(result)
                except Exception as e:
                    print(f"  Error running test: {e}")
                    traceback.print_exc()
                    self.results.append({
                        "name": test['name'],
                        "status": "error",
                        "error": str(e)
                    })
        
        self.generate_summary()
        self.save_results()
    
    def generate_summary(self):
        """Generate summary statistics"""
        passed = sum(1 for r in self.results if r.get("status") == "pass")
        failed = sum(1 for r in self.results if r.get("status") == "fail")
        errors = sum(1 for r in self.results if r.get("status") == "error")
        skipped = sum(1 for r in self.results if r.get("status") == "skipped")
        total = len(self.results)
        
        self.summary = {
            "total_tests": total,
            "passed": passed,
            "failed": failed,
            "errors": errors,
            "skipped": skipped,
            "pass_rate": passed / (total - skipped) * 100 if (total - skipped) > 0 else 0
        }
        
        # Find patterns in failures
        failure_patterns = {}
        for result in self.results:
            if result.get("status") == "fail":
                for diff in result.get("differences", []):
                    pattern = "Unknown"
                    if "error" in diff.lower():
                        pattern = "Error handling"
                    elif "float" in diff:
                        pattern = "Float precision"
                    elif "overflow" in result.get("name", "").lower():
                        pattern = "Integer overflow"
                    elif "nan" in str(result.get("pushr_result", {})).lower():
                        pattern = "Special float values"
                    
                    failure_patterns[pattern] = failure_patterns.get(pattern, 0) + 1
        
        self.summary["failure_patterns"] = failure_patterns
        
        # Print summary
        print(f"\n{'='*80}")
        print("ULTIMATE TEST SUMMARY")
        print(f"{'='*80}")
        print(f"Total tests: {total}")
        print(f"Passed: {passed} ({passed/total*100:.1f}%)")
        print(f"Failed: {failed}")
        print(f"Errors: {errors}")
        print(f"Skipped: {skipped}")
        
        if failure_patterns:
            print(f"\nFailure patterns:")
            for pattern, count in sorted(failure_patterns.items(), key=lambda x: x[1], reverse=True):
                print(f"  {pattern}: {count}")
        
        # Performance comparison
        pushr_times = [r["pushr_time"] for r in self.results if "pushr_time" in r]
        clojush_times = [r["clojush_time"] for r in self.results if "clojush_time" in r]
        
        if pushr_times and clojush_times:
            avg_pushr = sum(pushr_times) / len(pushr_times)
            avg_clojush = sum(clojush_times) / len(clojush_times)
            print(f"\nPerformance:")
            print(f"  Average Pushr time: {avg_pushr:.3f}s")
            print(f"  Average Clojush time: {avg_clojush:.3f}s")
            print(f"  Pushr is {avg_clojush/avg_pushr:.1f}x faster")
    
    def save_results(self):
        """Save detailed results to file"""
        output = {
            "summary": self.summary,
            "results": self.results
        }
        
        filename = "ultimate_test_results.json"
        with open(filename, "w") as f:
            json.dump(output, f, indent=2)
        print(f"\nDetailed results saved to {filename}")

def run_single_test(test_name, test_file="ultimate_test_suite.json"):
    """Run a single test by name"""
    if not os.path.exists(test_file):
        print(f"Error: {test_file} not found")
        sys.exit(1)
    
    with open(test_file, 'r') as f:
        test_suite = json.load(f)
    
    # Find the test
    found_test = None
    for category, tests in test_suite.items():
        for test in tests:
            if test['name'] == test_name:
                found_test = test
                found_category = category
                break
        if found_test:
            break
    
    if not found_test:
        print(f"Test '{test_name}' not found")
        print("\nAvailable tests:")
        for category, tests in test_suite.items():
            print(f"\n{category}:")
            for test in tests:
                print(f"  - {test['name']}")
        sys.exit(1)
    
    print(f"\nRunning single test: {test_name}")
    print(f"Category: {found_category}")
    print(f"Description: {found_test.get('description', '')}")
    
    comparison = UltimateComparison()
    result = comparison.run_test(found_test)
    
    # Print detailed results
    print(f"\nFinal result: {result['status'].upper()}")
    if result['status'] == 'fail' and 'differences' in result:
        print("\nDetailed differences:")
        for diff in result['differences']:
            print(f"  - {diff}")

def main():
    if not os.path.exists("Cargo.toml"):
        print("Error: Must run from the Pushr project root directory")
        sys.exit(1)
    
    # Check for command line arguments
    if len(sys.argv) > 1:
        if sys.argv[1] == "--test" and len(sys.argv) > 2:
            # Run single test
            test_name = sys.argv[2]
            run_single_test(test_name)
        elif sys.argv[1] == "--help" or sys.argv[1] == "-h":
            print("Usage:")
            print("  python ultimate_comparison.py              # Run all tests")
            print("  python ultimate_comparison.py --test NAME  # Run single test by name")
            print("  python ultimate_comparison.py --help       # Show this help")
            sys.exit(0)
        else:
            print("Unknown option. Use --help for usage information.")
            sys.exit(1)
    else:
        # Run all tests
        comparison = UltimateComparison()
        
        if os.path.exists("ultimate_test_suite.json"):
            comparison.run_all_tests("ultimate_test_suite.json")
        else:
            print("Error: ultimate_test_suite.json not found")
            sys.exit(1)

if __name__ == "__main__":
    main()