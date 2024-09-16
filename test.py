import requests
from random import randint

url = 'http://localhost:6543/knapsack'

capacity = randint(10,1000)
weights = []
values = []

for _ in range(0,100):
    weights.append(randint(5,100))
    values.append(randint(5,100))

data = {'problem': {'capacity': capacity, 'weights': weights, 'values': values}}

r = requests.post(url, data=data)


# requests.get(url + '/hhh')
