import requests
import json
import matplotlib.pyplot as plt
import numpy as np

# הגדר את כתובת ה-URL של השרת שלך
base_url = 'http://localhost:8080/api'  # שים לב שהכתובת משתנה לפי כתובת השרת שלך

# רשימת הגדלים של המטריצות שנרצה לבדוק

proof_gen_time = []
proof_verif_time = []
fibonacci_50 = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610,
    987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025,
    121393, 196418, 317811, 514229, 832040, 1346269, 2178309, 3524578,
    5702887, 9227465, 14930352, 24157817, 39088169, 63245986, 102334155,
    165580141, 267914296, 433494437, 701408733, 1134903170, 1836311903,
    2971215073, 4807526976, 7778742049
]
fibonacci_93 = [
    0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597,
    2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418,
    317811, 514229, 832040, 1346269, 2178309, 3524578, 5702887, 9227465,
    14930352, 24157817, 39088169, 63245986, 102334155, 165580141, 267914296,
    433494437, 701408733, 1134903170, 1836311903, 2971215073, 4807526976,
    7778742049, 12586269025, 20365011074, 32951280099, 53316291173, 86267571272,
    139583862445, 225851433717, 365435296162, 591286729879, 956722026041,
    1548008755920, 2504730781961, 4052739537881, 6557470319842, 10610209857723,
    17167680177565, 27777890035288, 44945570212853, 72723460248141, 117669030460994,
    190392490709135, 308061521170129, 498454011879264, 806515533049393,
    1304969544928657, 2111485077978050, 3416454622906707, 5527939700884757,
    8944394323791464, 14472334024676221, 23416728348467685, 37889062373143906,
    61305790721611591, 99194853094755497, 160500643816367088, 259695496911122585,
    420196140727489673, 679891637638612258, 1100087778366101931, 1779979416004714189,
    2880067194370816120, 4660046610375530309, 7540113804746346429
]

array_0_to_93 = []



try:
    for index, value in enumerate(fibonacci_93):
        array_0_to_93.append(index)
        print(f"Index: {index}, Value: {value}")
        prove_input = {
            "a": 0,
            "b": 1,
            "result": value,
            "num_of_rounds": index
        }

        # שלח בקשה לשרת ליצירת הוכחה
        response = requests.post(f'{base_url}/fibbonaci/prove', json=prove_input)

        if response.status_code == 200:
            prove_output = response.json()
            proof_gen_time.append(prove_output['proving_time'])


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
            proof_gen_time.append(0)
            proof_verif_time.append(0)
except Exception as e:
    print(f"An error occurred: {e}")
    # חיתוך matrix_sizes לפי אורך proof_verif_time
    fibonacci_93 = fibonacci_93[:len(proof_verif_time)]
    proof_gen_time = proof_gen_time[:len(proof_verif_time)]
    proof_verif_time = proof_verif_time[:len(proof_verif_time)]
    array_0_to_93 =array_0_to_93[:len(proof_verif_time)]

finally:
    # כתיבת התוצאות לקובץ JSON
    print("Finished processing all matrices and results saved to 'results.json'")
    results = {
        'number of elements': array_0_to_93,
        'fibonacci_93': fibonacci_93,
        'proof_gen_time': proof_gen_time,
        'proof_verif_time': proof_verif_time
    }
    with open('results1.json', 'w') as f:
        json.dump(results, f, indent=4)
    # יצירת גרפים
    print(array_0_to_93)
    print(proof_gen_time)
    print(proof_verif_time)


# גרף 2: זמן יצירת הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(array_0_to_93, proof_gen_time, marker='o', color='green')
    plt.xlabel('number of rounds')
    plt.ylabel('Proof Generation Time (ms)')
    plt.title('Proof Generation Time vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.show()

    # גרף 3: זמן אימות הוכחה מול גודל מטריצה
    plt.figure(figsize=(10, 7))
    plt.plot(array_0_to_93, proof_verif_time, marker='o', color='red')
    plt.xlabel('number of rounds')
    plt.ylabel('Proof Verification Time (ms)')
    plt.title('Proof Verification Time vs. number of rounds')
    plt.xticks(rotation=45)
    plt.tight_layout()
    plt.ylim(0,max(proof_verif_time)+0.01)

    plt.show()
