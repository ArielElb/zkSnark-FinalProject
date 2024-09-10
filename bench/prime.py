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
number_of_rounds =list(range(1, 50, 1)) #set the size of power of 2 for the matrixes you want
num_constraints = []
proof_gen_time = []
proof_set_up_time = []
num_variables = []
proof_verif_time = []
j_table = []
try:
    for rounds in number_of_rounds:
        random_32_bit_number = random.randint(0, 2 ** 32 - 1)
        power_of_2_size=pow(2, rounds) #we calculate with powers of 2
        print(f"Processing round: {rounds}")
        print(f"Processing the round in power of 2: {power_of_2_size}")
        print(random_32_bit_number)
        print(power_of_2_size)

        prove_input = {
            'x': random_32_bit_number, #Seed number (x)
            'i': power_of_2_size, #Number of rounds (i)
        }

        # שלח בקשה לשרת ליצירת הוכחה
        response = requests.post(f'{base_url}/prime_arkworks/prove', json=prove_input)

        if response.status_code == 200:
            prove_output = response.json()
            num_constraints.append(prove_output['num_constraints'])
            proof_gen_time.append(prove_output['proving_time'])
            proof_set_up_time.append(prove_output['setup_time'])
            num_variables.append(prove_output['num_variables'])
            j_table.append(prove_output['j'])
            # שלח בקשה לשרת לאימות הוכחה
            verify_input = {
                'x': random_32_bit_number, #Seed number (x)
                'j': prove_output['j'],
                'proof': prove_output['proof'],
                'pvk': prove_output['pvk'],
            }

            response = requests.post(f'{base_url}/prime_arkworks/verify', json=verify_input)

            if response.status_code == 200:
                verify_output = response.json()
                proof_verif_time.append(verify_output['verifying_time'])
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
    with open('results2.json', 'w') as f:
        json.dump(results, f, indent=4)

    print("Finished processing all matrices and results saved to 'resultsMatrix.json'")

    # יצירת גרפים
    print(number_of_rounds)

    # גרף 1: כמות הסיבובים מול מספר הקונסטריינס
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, num_constraints, marker='o')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('Circuit Size (# of constraints)')
    plt.title('Circuit Size vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # גרף 2: כמות הסיבובים מול מספר המשתנים
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, num_variables, marker='o', color='green')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('number of variables')
    plt.title('number of variables vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()
    # גרף 3: כמות הסיבובים מול זמן הsetup
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_set_up_time, marker='o', color='green')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('Proof  set up Time (seconds)')
    plt.title('Proof Generation set up  Time vs. number of rounds ')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()
    # גרף 4: כמות הסיבובים מול זמן ההוכחה
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_gen_time, marker='o', color='green')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('Proof Generation Time (seconds)')
    plt.title('Proof Generation Time vs. number of rounds ')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()
    # גרף 5: כמות הסיבובים מול j
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, j_table, marker='o', color='green')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('prime_round')
    plt.title('prime_round Time vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()
    # גרף 3: זמן אימות הוכחה מול כמות הסיבובים
    plt.figure(figsize=(10, 7))
    plt.plot(number_of_rounds, proof_verif_time, marker='o', color='red')
    plt.xlabel('number of rounds in power of 2')
    plt.ylabel('Proof Verification Time (seconds)')
    plt.title('Proof Verification Time vs. Matrix Size')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.ylim(0,max(proof_verif_time)+0.01)

    plt.show()
