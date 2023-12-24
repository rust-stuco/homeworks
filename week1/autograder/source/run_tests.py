import os
import unittest
from gradescope_utils.autograder_utils.json_test_runner import JSONTestRunner


if __name__ == "__main__":
    # Gather all of the tests in the /autograder/source/tests/ directory
    suite = unittest.defaultTestLoader.discover("tests")

    with open("/autograder/results/results.json", "w") as f:
        # All tests need to run with the test crate as the root
        os.chdir("/autograder/source/primerlab")
        # Run all tests and send json output to results.json
        JSONTestRunner(visibility="visible", stream=f).run(suite)
