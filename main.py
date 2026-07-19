import func.func as fc

def menu() :
  while True :
    print("====================================")
    print("[1] View Passwords")
    print("[2] Add New Entry")
    print("[3] Delete Entry")
    print("[4] Edit Entry")
    print("[5] Password Generator")
    print("[6] Exit")
    choice = input("===> :")
    result = fc.ec.ec(choice)
    if result == None :
      continue
    choice = int(choice)
    if choice == 1 : 
      fc.vp()
      continue
    if choice == 2: 
      fc.ae()
      continue 
    if choice == 3 : 
     fc.dl()
     continue
    if choice == 4 : 
      fc.ee()
      continue
    if choice == 5 : 
      fc.pd()
      continue
    if choice == 6 : 
      exit() 

menu()
