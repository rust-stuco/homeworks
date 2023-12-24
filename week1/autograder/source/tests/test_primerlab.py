import subprocess
import unittest
from gradescope_utils.autograder_utils.decorators import weight


class PrimerLabTest(unittest.TestCase):
    def run_cargo_test(self, test_name, cmd):
        print(f"Testing {test_name}...")
        # Runs `cargo test` in a subprocess
        test = subprocess.Popen(
            cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT
        )
        # Capture the output of the subprocess test
        output = test.stdout.read().strip().lower().decode()
        # Show student the output of the test
        print(output)
        # Check for any errors in output
        if "error" in output or "test result: FAILED" in output:
            self.assertTrue(False)

    @weight(48)
    def test_compiles(self):
        """Testing compilation of exercises"""
        self.run_cargo_test("compilation", "cargo test exercises::")

    @weight(2)
    def test_it_works(self):
        """Testing compilation of functions"""
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
