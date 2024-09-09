import requests
import json
import matplotlib.pyplot as plt
import numpy as np

# הגדר את כתובת ה-URL של השרת שלך
base_url = 'http://localhost:8080/api'  # שים לב שהכתובת משתנה לפי כתובת השרת שלך

# פונקציה ליצירת מטריצה בגודל n x n עם ערכים אקראיים
def generate_matrix(n):
    return [[1 for j in range(n)] for i in range(n)]


# רשימת הגדלים של המטריצות שנרצה לבדוק
matrix_sizes =list(range(1, 22, 1)) #set the size of power of 2 for the matrixes you want
circuit_size = []
proof_gen_time = []
proof_verif_time = []

try:
    for size in matrix_sizes:
        power_of_2_size=pow(2, size) #we calculate with powers of 2
        matrix_a = generate_matrix(power_of_2_size)
        matrix_b = generate_matrix(power_of_2_size)
        print(f"Processing matrix of power of 2 size: {size}")
        print(f"Processing matrix in size: {power_of_2_size}x{power_of_2_size}")

        prove_input = {
            'size': power_of_2_size,
            'matrix_a': matrix_a,
            'matrix_b': matrix_b
        }

        # שלח בקשה לשרת ליצירת הוכחה
        response = requests.post(f'{base_url}/matrix_prove/prove', json=prove_input)

        if response.status_code == 200:
            prove_output = response.json()
            circuit_size.append(prove_output['num_constraints'])
            proof_gen_time.append(prove_output['proving_time'])

            # שלח בקשה לשרת לאימות הוכחה
            verify_input = {
                'pvk': prove_output['pvk'],
                'proof': prove_output['proof'],
                'hash_a': prove_output['hash_a'],
                'hash_b': prove_output['hash_b'],
                'hash_c': prove_output['hash_c']
            }

            response = requests.post(f'{base_url}/matrix_prove/verify', json=verify_input)

            if response.status_code == 200:
                verify_output = response.json()
                proof_verif_time.append(verify_output['verifying_time'])
            else:
                print("wrong2")
                proof_verif_time.append(0)
        else:
            print("wrong1")
            circuit_size.append(0)
            proof_gen_time.append(0)
            proof_verif_time.append(0)
except Exception as e:
    print(f"An error occurred: {e}")
    # חיתוך matrix_sizes לפי אורך proof_verif_time
    matrix_sizes = matrix_sizes[:len(proof_verif_time)]
    circuit_size = circuit_size[:len(proof_verif_time)]
    proof_gen_time = proof_gen_time[:len(proof_verif_time)]
    proof_verif_time = proof_verif_time[:len(proof_verif_time)]

finally:
    # כתיבת התוצאות לקובץ JSON
    results = {
        'matrix_sizes': matrix_sizes,
        'circuit_size': circuit_size,
        'proof_gen_time': proof_gen_time,
        'proof_verif_time': proof_verif_time
    }
    with open('results2.json', 'w') as f:
        json.dump(results, f, indent=4)

    print("Finished processing all matrices and results saved to 'resultsMatrix.json'")

    # יצירת גרפים
    print(matrix_sizes)
    print(circuit_size)
    print(proof_gen_time)
    print(proof_verif_time)
    # גרף 1: גודל מעגל מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(matrix_sizes, circuit_size, marker='o')
    plt.xlabel('Matrix Size in power of 2')
    plt.ylabel('Circuit Size (# of constraints)')
    plt.title('Circuit Size vs. Matrix Size')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # גרף 2: זמן יצירת הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(matrix_sizes, proof_gen_time, marker='o', color='green')
    plt.xlabel('Matrix Size in power of 2')
    plt.ylabel('Proof Generation Time (seconds)')
    plt.title('Proof Generation Time vs. Matrix Size')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # גרף 3: זמן אימות הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(matrix_sizes, proof_verif_time, marker='o', color='red')
    plt.xlabel('Matrix Size in power of 2')
    plt.ylabel('Proof Verification Time (seconds)')
    plt.title('Proof Verification Time vs. Matrix Size')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.ylim(0,max(proof_verif_time)+0.01)

    plt.show()
