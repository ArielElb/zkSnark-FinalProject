import requests
import json
import matplotlib.pyplot as plt
import numpy as np
def generate_fibonacci(n):
    fibonacci_sequence = [0, 1]
    for i in range(2, n):
        next_number = fibonacci_sequence[-1] + fibonacci_sequence[-2]
        fibonacci_sequence.append(next_number)
    return fibonacci_sequence

fibonacci_numbers = generate_fibonacci(200)
print(fibonacci_numbers)

# הגדר את כתובת ה-URL של השרת שלך
base_url = 'http://localhost:8080/api'  # שים לב שהכתובת משתנה לפי כתובת השרת שלך

proof_gen_time = []
proof_verif_time = []

array_indexes = []



try:
    for index in range(0, 187, 1):
        array_indexes.append(index)
        print(f"Index: {index}, Value: {fibonacci_numbers[index]}")
        prove_input = {
            "a": 0,
            "b": 1,
            "result": str(fibonacci_numbers[index]),
            "num_of_rounds": index
        }

        # שלח בקשה לשרת ליצירת הוכחה
        response = requests.post(f'{base_url}/fibbonaci/prove', json=prove_input)

        if response.status_code == 200:
            prove_output = response.json()
            proof_gen_time.append(prove_output['proving_time'])
            print(prove_output['proving_time'])

            # שלח בקשה לשרת לאימות הוכחה
            verify_input = {
                'pvk': prove_output['pvk'],
                'proof': prove_output['proof'],
                "a": 0,
                "b": 1
            }

            response = requests.post(f'{base_url}/fibbonaci/verify', json=verify_input)

            if response.status_code == 200:
                verify_output = response.json()
                proof_verif_time.append(verify_output['verifying_time'])
                print(verify_output)
            else:
                print("wrong2")
                proof_verif_time.append(0)
        else:
            print("wrong1")
            print(response.status_code)
            proof_gen_time.append(0)
            proof_verif_time.append(0)
except Exception as e:
    print(f"An error occurred: {e}")
    # חיתוך matrix_sizes לפי אורך proof_verif_time
    proof_gen_time = proof_gen_time[:len(proof_verif_time)]
    proof_verif_time = proof_verif_time[:len(proof_verif_time)]
    array_indexes =array_indexes[:len(proof_verif_time)]

finally:
    # כתיבת התוצאות לקובץ JSON
    print("Finished processing all matrices and results saved to 'resultsMatrix.json'")
    results = {
        'number of elements': array_indexes,
        'proof_gen_time': proof_gen_time,
        'proof_verif_time': proof_verif_time
    }
    with open('results1.json', 'w') as f:
        json.dump(results, f, indent=4)
    # יצירת גרפים
    print(array_indexes)
    print(proof_gen_time)
    print(proof_verif_time)


# גרף 2: זמן יצירת הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(array_indexes, proof_gen_time, marker='o', color='green')
    plt.xlabel('number of rounds')
    plt.ylabel('Proof Generation Time (seconds)')
    plt.title('Proof Generation Time vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # גרף 3: זמן אימות הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(array_indexes, proof_verif_time, marker='o', color='red')
    plt.xlabel('number of rounds')
    plt.ylabel('Proof Verification Time (seconds)')
    plt.title('Proof Verification Time vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.ylim(0,max(proof_verif_time)+0.01)

    plt.show()
