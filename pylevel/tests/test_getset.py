import os
import unittest
import tempfile

import pylevel

class TestGetSet(unittest.TestCase):
    def setUp(self):
        self.db = pylevel.DB(tempfile.mktemp(), create_if_missing=True)

    def test_getset(self):
        self.assertEqual(self.db.get(b'key'), None)

        self.db.put(b'key', b'value')
        self.assertEqual(self.db.get(b'key'), b'value')

        self.db.put(b'key', b'value2')
        self.assertEqual(self.db.get(b'key'), b'value2')

        self.db.delete(b'key')
        self.assertEqual(self.db.get(b'key'), None)

        self.db.flush()

    def tearDown(self):
        self.db.close()
