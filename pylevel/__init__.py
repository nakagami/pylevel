from .rslevel import DB as InnterDB
from .rslevel import LockError

class DB:
    def __init__(self, dirname, create_if_missing=False):
        self.db = InnterDB(dirname, create_if_missing=create_if_missing)

    def get(self, k):
        if v:= self.db.get(k):
            return bytes(v)
        return None

    def put(self, k, v):
        self.db.put(k, v)

    def delete(self, k):
        self.db.delete(k)

    def flush(self):
        self.db.flush()

    def close(self):
        del self.db
