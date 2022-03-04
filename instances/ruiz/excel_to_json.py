"""
Converter to make excel file to json to be used in Rust.
"""

from pydoc import importfile
import pandas as pd
import numpy as np
import json

SAVE_PATH = "json\\"


# Splits the dataframe into many and returns the list of all dataframes with last element as remainder
def split_dataframe(df: pd.DataFrame, height: int, width: int, n: int) -> list:
    df_list = []
    for index in range(0, n):
        df_keep, df = np.split(df, [height], axis=0)
        if not well_behaved(df_keep, width):
            print("Error: Something went wrong with the height")
            print(df_keep)
        df_list.append(df_keep)
    if not df.empty:
        df_list.append(df)
    return df_list

# Tests if all but first entry in the first row is zero values
def well_behaved(df: pd.DataFrame, width: int) -> bool:
    well_behaved = True
    for index in range(1,width):
        if df.iat[0,index] != 0:
            well_behaved = False
    return well_behaved

# Interperets dataframe and makes json file.
def to_json(df: pd.DataFrame, save_as: str) -> dict:
    dic = {}
    dic["products"] = int(df.iloc[0,0])
    dic["stages"] = int(df.iloc[1,0])
    dic["machines"] = []
    for index in range(dic["stages"]):
        dic["machines"].append(int(df.iloc[2,index]))
    dic["production_times"] = []
    for index in range(dic["products"]):
        product = []
        for index2 in range(dic["stages"]):
            product.append(int(df.iloc[3+index, index2]))
        dic["production_times"].append(product)
    dic["setup_times"] = []
    for stage in range(dic["stages"]):
        stage_setup = []
        for index in range(dic["products"]*stage, dic["products"]*stage+dic["products"]):
            setup = []
            for index2 in range(dic["products"]):
                setup.append(int(df.iloc[3+dic["products"]+index, index2]))
            stage_setup.append(setup)
    dic["setup_times"].append(stage_setup)
    with open(f"{SAVE_PATH}{save_as}.json", "w", encoding="utf-8") as f:
        json.dump(dic, f, ensure_ascii=False)
    return dic

# Extraction of dataframes from excel files
width = 20

df = pd.read_excel(io="n=20\m=2,4\\n=20, m=2,4.xls", header=None)
height = 63
n20m2 = split_dataframe(df=df, height=height, width=width, n=80)
df = n20m2.pop()
height = 103
n20m4 = split_dataframe(df=df, height=height, width=width, n=80)

df = pd.read_excel(io="n=20\m=8\\n=20, m=8.xls", header=None)
height = 183
n20m8 = split_dataframe(df=df, height=height, width=width, n=80)

width = 50

df = pd.read_excel(io="n=50\m=2\\n=50, m=2.xls", header=None)
height = 153
n50m2 = split_dataframe(df=df, height=height, width=width, n=80)

df = pd.read_excel(io="n=50\m=4\\n=50, m=4.xls")
height = 253
n50m4 = split_dataframe(df=df, height=height, width=width, n=80)

df = pd.read_excel(io="n=50\m=8\\n=50, m=8.xls", header=None)
height = 453
n50m8 = split_dataframe(df=df, height=height, width=width, n=80)

width = 80

df = pd.read_excel(io="n=80\\n=80,m=2\\n=80,m=2.xls", header=None)
height = 243
n80m2 = split_dataframe(df=df, height=height, width=width, n=80)

df = pd.read_excel(io="n=80\\n=80,m=4\\Constant\\n=80, m=4, Con.xls", header=None)
height = 403
n80m4_constant = split_dataframe(df=df, height=height, width=width, n=40)

df = pd.read_excel(io="n=80\\n=80,m=4\\Variable\\n=80, m=4, Var.xls", header=None)
n80m4_variable = split_dataframe(df=df, height=height, width=width, n=40)


# Conversion of dataframes to .json

for index in range(80):
    to_json(df=n20m2[index], save_as=f"n20m2-{index+1}")