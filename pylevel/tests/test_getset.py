import os
import unittest
import tempfile

import pylevel

class TestGetSet(unittest.TestCase):
    def setUp(self):
        self.db = pylevel.DB(tempfile.mktemp(), create_if_missing=True)

    def test_getset(self):
        self.assertEqual(self.db.get(b'key'), None)

    def tearDown(self):
        pass
        # TODO: close db
