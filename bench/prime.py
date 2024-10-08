import time

import requests
import json
import matplotlib.pyplot as plt
import numpy as np
import random

# הגדר את כתובת ה-URL של השרת שלך
base_url = 'http://localhost:8080/api'  # שים לב שהכתובת משתנה לפי כתובת השרת שלך
# פונקציה ליצירת מטריצה בגודל n x n עם ערכים אקראיים
def generate_matrix(n):
    return [[1 for j in range(n)] for i in range(n)]


# רשימת הגדלים של המטריצות שנרצה לבדוק
number_of_rounds =list(range(1, 7, 1)) #set the size of power of 2 for the matrixes you want
num_constraints = []
proof_gen_time = []
proof_set_up_time = []
num_variables = []
proof_verif_time = []
j_table = []
try:
    for rounds in number_of_rounds:
        time.sleep(0.5)
        for i in range(10):
            while True:
                rounds_power_2=2**rounds
                pre_power_of2=2**(rounds-1)
                random_i_bit_number = random.randint(2**pre_power_of2 , 2**rounds_power_2 - 1)
                print(f"Processing round: {rounds} in pwer of 2- {rounds_power_2}")
                print(f"Processing number: {random_i_bit_number}")
                prove_input = {
                    'x': random_i_bit_number, #Seed number (x)
                    'i': 32, #Number of rounds (i)
                }
    
                # שלח בקשה לשרת ליצירת הוכחה
                response = requests.post(f'{base_url}/prime_arkworks/prove', json=prove_input)
    
                if response.status_code == 200:
                    prove_output = response.json()
                    print(prove_output)
                    if prove_output['found_prime'] == False:
                        continue
                    num_constraints.append(prove_output['num_constraints'])
                    proof_gen_time.append(prove_output['proving_time'])
                    proof_set_up_time.append(prove_output['setup_time'])
                    num_variables.append(prove_output['num_variables'])
                    j_table.append(prove_output['j'])
                    # שלח בקשה לשרת לאימות הוכחה
                    verify_input = {
                        'x': random_i_bit_number, #Seed number (x)
                        'j': prove_output['j'],
                        'proof': prove_output['proof'],
                        'pvk': prove_output['pvk'],
                    }
    
                    response = requests.post(f'{base_url}/prime_arkworks/verify', json=verify_input)
    
                    if response.status_code == 200:
                        verify_output = response.json()
                        proof_verif_time.append(verify_output['verifying_time'])
                        break
                    else:
                        print("wrong2")
                        proof_verif_time.append(0)
                else:
                    print("wrong1")

except Exception as e:
    print(f"An error occurred: {e}")
    # חיתוך matrix_sizes לפי אורך proof_verif_time
    number_of_rounds = number_of_rounds[:len(proof_verif_time)]
    num_constraints = num_constraints[:len(proof_verif_time)]
    proof_gen_time = proof_gen_time[:len(proof_verif_time)]
    proof_set_up_time = proof_set_up_time[:len(proof_verif_time)]
    num_variables = num_variables[:len(proof_verif_time)]
    j_table = j_table[:len(proof_verif_time)]


finally:
    # כתיבת התוצאות לקובץ JSON
    results = {
        'number_of_rounds': number_of_rounds,
        'num_constraints': num_constraints,
        'num_variables': num_variables,
        'proof_set_up_time': proof_set_up_time,
        'proof_gen_time': proof_gen_time,
        'j_table': j_table,
        'proof_verif_time': proof_verif_time
    }
    with open('results3.json', 'w') as f:
        json.dump(results, f, indent=4)

    print("Finished processing all matrices and results saved to 'resultsMatrix.json'")

    # יצירת גרפים
    print(number_of_rounds)

    # גרף 1: כמות הסיבובים מול מספר הקונסטריינס
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, num_constraints, marker='o')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('Circuit Size (# of constraints)')
    plt.title('Circuit Size vs. the number of bits of the number')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.show()

    # גרף 2: כמות הסיבובים מול מספר המשתנים
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, num_variables, marker='o', color='green')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('number of variables')
    plt.title('number of variables vs. the number of bits of the number')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.show()
    

    # גרף 3: כמות הסיבובים מול זמן הsetup
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_set_up_time, marker='o', color='green')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('Proof  set up Time (seconds)')
    plt.title('Proof Generation set up  Time vs.the number of bits of the number ')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.show()
    # גרף 4: כמות הסיבובים מול זמן ההוכחה
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_gen_time, marker='o', color='green')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('Proof Generation Time (seconds)')
    plt.title('Proof Generation Time vs. the number of bits of the number ')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.show()

    # גרף 5: כמות הסיבובים מול j
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, j_table, marker='o', color='green')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('the number of rounds we use')
    plt.title('the number of rounds we use vs. the number of bits of the number')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.show()
    # גרף 6: זמן אימות הוכחה מול כמות הסיבובים
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_verif_time, marker='o', color='red')
    plt.xlabel('the number of bits of the number')
    plt.ylabel('Proof Verification Time (seconds)')
    plt.title('Proof Verification Time vs. the number of bits of the number')
    plt.xticks(number_of_rounds, [f'$2^{x}$' for x in number_of_rounds])
    plt.tight_layout()
    plt.ylim(0,max(proof_verif_time)+0.01)

    plt.show()

