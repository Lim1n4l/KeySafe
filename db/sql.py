import sqlite3 as sql
from pathlib import Path as P

d = P(__file__).resolve().parent # current directory
db_path = d / "data.db"

def db () : 
  connect = sql.connect(db_path)
  cursor = connect.cursor()
  cursor.execute("CREATE TABLE IF NOT EXISTS info (" \
  "id INTEGER PRIMARY KEY AUTOINCREMENT ," \
  "user TXT ," \
  "password TXT " \
  ")")
  connect.commit()
  return connect , cursor

cnt , crs = db()

