"""
Converter to make excel file to json to be used in Rust.
"""

from email import header
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

# Tests if all but first entry in the first row are zero values
def well_behaved(df: pd.DataFrame, width: int) -> bool:
    well_behaved = True
    for index in range(1,width):
        if df.iat[0,index] != 0:
            well_behaved = False
    return well_behaved

# Interperets dataframe and makes json file.
def to_json(df: pd.DataFrame, save_as: str) -> dict:
    dic = {}
    dic["jobs"] = int(df.iloc[0,0])
    dic["stages"] = int(df.iloc[1,0])
    dic["machines"] = []
    for index in range(dic["stages"]):
        dic["machines"].append(int(df.iloc[2,index]))
    dic["processing_times"] = []
    for index in range(dic["jobs"]):
        product = []
        for index2 in range(dic["stages"]):
            product.append(int(df.iloc[3+index, index2]))
        dic["processing_times"].append(product)
    dic["setup_times"] = []
    for stage in range(dic["stages"]):
        stage_setup = []
        for index in range(dic["jobs"]*stage, dic["jobs"]*stage+dic["jobs"]):
            setup = []
            for index2 in range(dic["jobs"]):
                setup.append(int(df.iloc[3+dic["jobs"]+index, index2]))
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

df = pd.read_excel(io="n=50\m=4\\n=50, m=4.xls", header=None)
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

df = pd.read_excel(io="n=80\\n=80,m=8\\SDST25\\n=80, m=8, 25_.xls", header=None)
height = 723
n80m8_25 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=80\\n=80,m=8\\SDST50\\n=80, m=4, 50_.xls", header=None)
n80m8_50 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=80\\n=80,m=8\\SDST100\\n=80, m=4, 100_.xls", header=None)
n80m8_100 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=80\\n=80,m=8\\SDST125\\n=80, m=4, 125_.xls", header=None)
n80m8_125 = split_dataframe(df=df, height=height, width=width, n=20)

width = 120

df = pd.read_excel(io="n=120\\n=120,m=2\\Constant\\n=120, m=2, Con.xls", header=None)
height = 363
n120m2_constant = split_dataframe(df=df, height=height, width=width, n=40)

df = pd.read_excel(io="n=120\\n=120,m=2\\Variable\\n=120, m=2, Var.xls", header=None)
n120m2_variable = split_dataframe(df=df, height=height, width=width, n=40)

df = pd.read_excel(io="n=120\\n=120,m=4\\SDST25\\n=120, m=4, 25_.xls", header=None)
height = 603
n120m4_25 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=120\\n=120,m=4\\SDST50\\n=120, m=4, 50_.xls", header=None)
n120m4_50 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=120\\n=120,m=4\\SDST100\\n=120, m=4, 100_.xls", header=None)
n120m4_100 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=120\\n=120,m=4\\SDST125\\n=120, m=4, 125_.xls", header=None)
n120m4_125 = split_dataframe(df=df, height=height, width=width, n=20)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST25\\Constant\\n = 120, m = 8, 25_, Con.xls", header=None)
height = 1083
n120m8_25_constant = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST25\\Variable\\n=120, m = 8, 25_, Var.xls", header=None)
n120m8_25_variable = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST50\\Constant\\n=120, m=5, 50_, Con.xls", header=None) # Believe the m5 should have been m8
n120m8_50_constant = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST50\\Variable\\n=120, m=8, 50_, Var.xls", header=None)
n120m8_50_variable = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST100\\Constant\\n=120, m=8, 100_, Con.xls", header=None)
n120m8_100_constant = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST100\\Variable\\n=120, m=8, 100_, Var.xls", header=None)
n120m8_100_variable = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST125\\Constant\\n=120, m=8, 125_, Con.xls", header=None)
n120m8_125_constant = split_dataframe(df=df, height=height, width=width, n=10)

df = pd.read_excel(io="n=120\\n=120,m=8\\SDST125\\Variable\\n=120, m=8, 125_, Var.xls", header=None)
n120m8_125_variable = split_dataframe(df=df, height=height, width=width, n=10)


# Conversion of dataframes to .json

for index in range(80):
    if index < 9:
        to_json(df=n20m2[index], save_as=f"n20m2-0{index+1}")
        to_json(df=n20m4[index], save_as=f"n20m4-0{index+1}")
        to_json(df=n20m8[index], save_as=f"n20m8-0{index+1}")
        to_json(df=n50m2[index], save_as=f"n50m2-0{index+1}")
        to_json(df=n50m4[index], save_as=f"n50m4-0{index+1}")
        to_json(df=n50m8[index], save_as=f"n50m8-0{index+1}")
        to_json(df=n80m2[index], save_as=f"n80m2-0{index+1}")
    else:
        to_json(df=n20m2[index], save_as=f"n20m2-{index+1}")
        to_json(df=n20m4[index], save_as=f"n20m4-{index+1}")
        to_json(df=n20m8[index], save_as=f"n20m8-{index+1}")
        to_json(df=n50m2[index], save_as=f"n50m2-{index+1}")
        to_json(df=n50m4[index], save_as=f"n50m4-{index+1}")
        to_json(df=n50m8[index], save_as=f"n50m8-{index+1}")
        to_json(df=n80m2[index], save_as=f"n80m2-{index+1}")

for index in range(40):
    if index < 9:
        to_json(df=n80m4_constant[index], save_as=f"n80m4-0{index+1}")
        to_json(df=n80m4_variable[index], save_as=f"n80m4-{index+41}")
    else:
        to_json(df=n80m4_constant[index], save_as=f"n80m4-{index+1}")
        to_json(df=n80m4_variable[index], save_as=f"n80m4-{index+41}")

for index in range(20):
    if index < 9:
        to_json(df=n80m8_25[index], save_as=f"n80m8-0{index+1}")
        to_json(df=n80m8_25[index], save_as=f"n80m8-{index+1}")
        to_json(df=n80m8_50[index], save_as=f"n80m8-{index+21}")
        to_json(df=n80m8_100[index], save_as=f"n80m8-{index+41}")
        to_json(df=n80m8_125[index], save_as=f"n80m8-{index+61}")
    else:
        to_json(df=n80m8_25[index], save_as=f"n80m8-{index+1}")
        to_json(df=n80m8_50[index], save_as=f"n80m8-{index+21}")
        to_json(df=n80m8_100[index], save_as=f"n80m8-{index+41}")
        to_json(df=n80m8_125[index], save_as=f"n80m8-{index+61}")

for index in range(40):
    if index < 9:
        to_json(df=n120m2_constant[index], save_as=f"n120m2-0{index+1}")
        to_json(df=n120m2_variable[index], save_as=f"n120m2-{index+41}")
    else:
        to_json(df=n120m2_constant[index], save_as=f"n120m2-{index+1}")
        to_json(df=n120m2_variable[index], save_as=f"n120m2-{index+41}")

for index in range(20):
    if index < 9:
        to_json(df=n120m4_25[index], save_as=f"n120m4-0{index+1}")
        to_json(df=n120m4_50[index], save_as=f"n120m4-{index+21}")
        to_json(df=n120m4_100[index], save_as=f"n120m4-{index+41}")
        to_json(df=n120m4_125[index], save_as=f"n120m4-{index+61}")
    else:
        to_json(df=n120m4_25[index], save_as=f"n120m4-{index+1}")
        to_json(df=n120m4_50[index], save_as=f"n120m4-{index+21}")
        to_json(df=n120m4_100[index], save_as=f"n120m4-{index+41}")
        to_json(df=n120m4_125[index], save_as=f"n120m4-{index+61}")

for index in range(10):
    if index < 9:
        to_json(df=n120m8_25_constant[index], save_as=f"n120m8-0{index+1}")
        to_json(df=n120m8_25_variable[index], save_as=f"n120m8-{index+11}")
        to_json(df=n120m8_50_constant[index], save_as=f"n120m8-{index+21}")
        to_json(df=n120m8_50_variable[index], save_as=f"n120m8-{index+31}")
        to_json(df=n120m8_100_constant[index], save_as=f"n120m8-{index+41}")
        to_json(df=n120m8_100_variable[index], save_as=f"n120m8-{index+51}")
        to_json(df=n120m8_125_constant[index], save_as=f"n120m8-{index+61}")
        to_json(df=n120m8_125_variable[index], save_as=f"n120m8-{index+71}")
    else:
        to_json(df=n120m8_25_constant[index], save_as=f"n120m8-{index+1}")
        to_json(df=n120m8_25_variable[index], save_as=f"n120m8-{index+11}")
        to_json(df=n120m8_50_constant[index], save_as=f"n120m8-{index+21}")
        to_json(df=n120m8_50_variable[index], save_as=f"n120m8-{index+31}")
        to_json(df=n120m8_100_constant[index], save_as=f"n120m8-{index+41}")
        to_json(df=n120m8_100_variable[index], save_as=f"n120m8-{index+51}")
        to_json(df=n120m8_125_constant[index], save_as=f"n120m8-{index+61}")
        to_json(df=n120m8_125_variable[index], save_as=f"n120m8-{index+71}")
