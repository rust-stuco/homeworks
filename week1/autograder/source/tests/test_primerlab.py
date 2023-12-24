import subprocess, os
import unittest
from gradescope_utils.autograder_utils.decorators import weight


def contains_error(output):
    return "error" in output or "test result: FAILED" in output


def run_command(cmd):
    test = subprocess.Popen(
        cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
    )
    output = test.stdout.read().strip().lower().decode()
    return output


class PrimerLabTest(unittest.TestCase):
    def run_cargo_test(self, test_name, cmd):
        print(f"Testing {test_name}...")
        output = run_command(cmd)
        print(output)

        if contains_error(output):
            self.assertTrue(False)

    @weight(48)
    def test_compiles(self):
        """Testing Compilation of Exercises"""
        self.run_cargo_test("compilation", "cargo test exercises::")

    @weight(2)
    def test_it_works(self):
        """Testing Compilation of Functions"""
        self.run_cargo_test("it_works", "cargo test it_works")

    @weight(12)
    def test_is_prime(self):
        """Testing is_prime"""
        self.run_cargo_test(
            "is_prime", "cargo test is_prime && cargo test random_primes"
        )

    @weight(16)
    def test_nth_prime(self):
        """Testing nth_prime"""
        self.run_cargo_test("nth_prime", "cargo test --release nth_prime")

    @weight(11)
    def test_gcd(self):
        """Testing gcd"""
        self.run_cargo_test("gcd", "cargo test gcd")

    @weight(11)
    def test_fib(self):
        """Testing fib"""
        self.run_cargo_test("fib", "cargo test fib")
