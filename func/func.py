import time as tm
import db.sql as sql
import random as rd
import string as sr

class ER :
  def ec(self , inp) :
    opt = (1 , 2 , 3 , 4 , 5 , 6) # options
    try :
      inp = int(inp)
    except :
      print("[!] Invalid input Please enter a number")
      tm.sleep(1.5)
      return None
    if inp not in opt :
      print("[!] Invalid input. Please select a valid option from the menu.")
      tm.sleep(1.5)
      return None
    return True
  def idc(self , id) : # ID existence checker
    sql.crs.execute("SELECT id FROM info WHERE id = ? ;" , (id ,))
    row = sql.crs.fetchone()
    if row is None:
        return False
    return True
  def yn (self , ans) :
    if ans not in ("y" , "n") :
      print("[!]Invalid input try again")
      tm.sleep(1)
      return False 

ec = ER()

def ae() : # add entry
  user = input("Enter Username or Email\n===> :")
  password = input("Input Password \n===> :")
  print("User :" + user)
  print("Password :" + password)
  while True :
    ct = input("Do you want to continue ?\n===> :").lower().strip()
    if ct not in ("y" , "n") :
      print("[!] Invalid input Please select [Y/N]")
      tm.sleep(1.5)
      continue
    if ct == "y" :
      sql.cnt.execute("INSERT INTO info (user , password) VALUES (? , ?) ;" , (user , password ,))
      sql.cnt.commit()
      print("[+] Input saved successfully")
      tm.sleep(1.5)
      break
    if ct == "n" :
      print("[+] canceling the process")
      tm.sleep(2)
      break
def vp() : # view passwords
  sql.crs.execute("SELECT * FROM info ;")
  rows = sql.crs.fetchall()
  for row in rows:
    print(f"ID: {row[0]}")
    print(f"User: {row[1]}")
    print(f"Password: {row[2]}")
    print("-" * 30)
  tm.sleep(3)
def ee() : # edit entry
  vp()
  while True :
    choice = input("Select entry to edit[by id]\n===> :").strip()
    if not choice.isdigit() : 
      print("[!] Invalid input try again\n")
      tm.sleep(1)
      continue
    r = ec.idc(choice)
    if r == False :
      print("[!]Invalid input try again")
      tm.sleep(1)
      continue
    else :
      break
  while True :
    data = input("Enter what you want to edit [user/password]\nNote:enter (.)if you want to skip one\n===> :").strip()
    try :
      user, passw = data.split("/")
    except :
      print("[!] Use format: user/password")
      tm.sleep(2)
      continue
    if user == "." and passw == "." :
      print("[!] the user and password are " + user + passw)
      print("try again")
      tm.sleep(1)
      continue
    else :
      break
  if user != "." :
    sql.crs.execute("UPDATE info SET user = ? WHERE id = ? ;" , (user , choice ,))
    sql.cnt.commit()
    print("[+]user updated successfully") 
    tm.sleep(2)
  if passw != "." :
    sql.crs.execute("UPDATE info SET password = ? WHERE id = ? ;" , (passw , choice ,))
    sql.cnt.commit()
    print("[+]password updated susscafuly") 
    tm.sleep(2)
def dl() : # delete entry
  vp()
  print("Input the id of the Entry you want to delete")
  while True :
    ch = input("===> :").strip()
    if not ch.isdigit() :
      print("[!] Invalid input try again")
      tm.sleep(1.5)
      continue
    else :  
      sql.crs.execute("SELECT * FROM info WHERE id = ? ;", (ch,))
      row = sql.crs.fetchone()
      if row == None :
        print("[!] Invalid input There is no entery with the given id " + ch)
        tm.sleep(1.5)
        continue
      else : 
        print("---------------")
        print(f"ID: {row[0]}")
        print(f"User: {row[1]}")
        print(f"Password: {row[2]}")
        print("---------------")
        break
  while True :
    print("are you sure you want to delete it ? [y/n]")
    ch2 = input("===> :").strip().lower()
    if ch2 not in ("y" , "n") :
      print("[!] Invalid input try again")
      tm.sleep(1.5)
      continue
    if ch2 == "n" :
      print("[+] Stopping the deletion ...")
      tm.sleep(1.5) 
      break
    if ch2 == "y" :
      print("[+] Processing the deletion")
      sql.crs.execute("DELETE FROM info WHERE id = ? ;" , (ch,))
      sql.cnt.commit()
      tm.sleep(1.5)
      print("[+] Deletion complete")
      tm.sleep(1.5)
      break  
def pd() : # password generator
  parts = ""
  while True :
    print("Do you want uppercase ?[y/n]")
    ans = input("===> :").strip().lower()
    r = ec.yn(ans)
    if r == False :
      continue
    else :
      break
  if ans == "y" :
    parts += sr.ascii_uppercase
  while True :
    print("Do you want lowercase ?")
    ans = input("===> :")
    r = ec.yn(ans)
    if r == False :
      continue
    else :
      break
  if ans == "y" :
    parts += sr.ascii_lowercase
  while True :
    print("Do you want numbers ?")
    ans = input("==> :")
    r = ec.yn(ans)
    if r == False :
      continue
    else :
      break
  if ans == "y" :
    parts += sr.digits
  while True :
    print("Do you want symbols ?")
    ans = input("===> :")
    r = ec.yn(ans)
    if r == False :
      continue
    else :
      break
  if ans == "y" :
    parts += sr.punctuation
  while True :
    print("How long should the password be ?")
    long = input("===> :")
    try :
      long = int(long)
    except :
      print("[!] Invalid input")
      tm.sleep(1)
      continue
    break
  passwd = ""
  if not parts:
    print("[!] Select at least one option")
    tm.sleep(2)
    return
  for _ in range(long) :
    passwd += rd.choice(parts)
  print("[+] Password generated successfully")
  print("[+] password ==>:" + passwd)
  tm.sleep(2)
