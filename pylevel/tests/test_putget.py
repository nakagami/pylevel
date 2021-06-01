import os
import unittest
import tempfile

import pylevel

class TestPutGet(unittest.TestCase):
    def setUp(self):
        self.path = tempfile.mktemp()
        self.db = pylevel.DB(self.path, create_if_missing=True)

    def test_putget(self):
        with self.assertRaises(pylevel.LockError):
            pylevel.DB(self.path)

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
