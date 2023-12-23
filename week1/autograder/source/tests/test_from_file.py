import unittest
from gradescope_utils.autograder_utils.decorators import weight
import os, subprocess


def run_command(cmd):
    test = subprocess.Popen(cmd, shell=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    output = test.stdout.read().strip().lower().decode()
    return output


def contains_error(output):
    return "error" in output or "test result: FAILED" in output


class TestDiff(unittest.TestCase):
    def setUp(self):
        os.chdir("/autograder/source/primerlab")


    @weight(50)
    def test_compiles(self):
        print("Testing compilation...")
        output = run_command("cargo test exercises::")
        print(output)

        if contains_error(output):
            self.assertTrue(False)


    @weight(10)
    def test_doc_tests(self):
        print("Testing doc comment tests...")
        output = run_command("cargo test --doc")
        print(output)

        if contains_error(output):
            self.assertTrue(False)


    @weight(40)
    def test_functions(self):
        print("Testing functions...")
        output = run_command("cargo test tests:: --release")
        print(output)

        if contains_error(output):
            self.assertTrue(False)
