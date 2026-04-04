import os

# AG004: bare calls without module prefix
df = read_csv("data.csv")        # should be pd.read_csv
arr = array([1, 2, 3])           # should be np.array
data = loads('{"key": "val"}')   # should be json.loads
result = DataFrame({"a": [1]})   # should be pd.DataFrame

# AG005: module used but never imported
result = np.array([1, 2, 3])     # np not imported
data = pd.read_csv("file.csv")   # pd not imported
resp = requests.get("http://x")  # requests not imported

# These should NOT trigger:
from json import dumps
x = dumps({"a": 1})              # imported via from-import, OK

y = len([1, 2, 3])               # builtin, OK
z = print("hello")               # builtin, OK
