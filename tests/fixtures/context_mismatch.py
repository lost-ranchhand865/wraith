import pandas as pd

# AG006: contextual mismatches
df1 = pd.read_excel("data.csv")        # csv file with read_excel
df2 = pd.read_csv("report.xlsx")       # xlsx file with read_csv
df3 = pd.read_json("data.parquet")     # parquet file with read_json

# These should NOT trigger:
df4 = pd.read_csv("data.csv")          # correct
df5 = pd.read_excel("report.xlsx")     # correct
df6 = pd.read_json("config.json")      # correct
df7 = pd.read_csv("data")              # no extension, skip
