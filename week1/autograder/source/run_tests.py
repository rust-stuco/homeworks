import os
import unittest
from gradescope_utils.autograder_utils.json_test_runner import JSONTestRunner


if __name__ == "__main__":
    suite = unittest.defaultTestLoader.discover("tests")
    with open("/autograder/results/results.json", "w") as f:
        os.chdir("/autograder/source/primerlab")
        JSONTestRunner(visibility="visible", stream=f).run(suite)
