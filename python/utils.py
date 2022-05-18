import pandas as pd
import plotly.express as px
import plotly.graph_objects as go


# Get param dataframe given folder location and parameter name(s)
def get_params(folder, params):
    df = pd.read_csv(folder + "/params.csv", usecols=params)
    df.columns = df.columns.astype(str)

    return df


# Get dataframe with all results
def get_results(folder, names):
    df = pd.read_csv(folder + "/results.csv", names=names)
    df.columns = df.columns.astype(str)

    return df


# Calculates the relative deviation percentage for all problem instances
def calc_rpd(df):
    columns = list(df.columns.values)
    columns = [str(elem) for elem in columns]
    for col in columns:
        df[col + "_rpd"] = (df[col] / df[columns].min(axis=1) - 1) * 100

    return df


# Calculates mean and standard error of mean for all problem instance
def calc_stats(df):
    df = df.transpose()
    df["mean"] = df.mean(axis=1)
    df["sem"] = df.drop("mean", axis=1).apply(lambda x: x.sem(), axis=1)
    return df.filter(like="_rpd", axis=0)[["mean", "sem"]]


# Takes a dataframe with statistics to plot and returns a figure
def plot_from_df(df, xname, yname, new_names={}, rename=True):
    if rename:
        df.rename(index=lambda s: s.replace("_rpd", ""), inplace=True)

    if len(new_names) > 0:
        df.rename(index=new_names, inplace=True)

    fig = px.scatter(
        df,
        x=df.index,
        y="mean",
        width=500,
        height=400,
        template="simple_white",
        error_y="sem",
    )

    fig.update_layout(
        xaxis_type="category",
        xaxis_title=xname,
        yaxis_title=yname,
        font_family="Times New Roman",
    )

    fig.update_xaxes(mirror=True)
    fig.update_yaxes(mirror=True)

    fig.update_traces(marker_size=10)

    return fig


# Create a plot from two different problem folders
def merge_and_plot(folders, names, x_title, y_title):
    dfs = []
    for fname, colname in zip(folders, names):
        dfs.append(get_results(fname, [colname]))

    res = pd.concat(dfs, axis=1)

    rpd = calc_rpd(res)

    stats = calc_stats(rpd)

    fig = plot_from_df(stats, x_title, y_title)

    return fig


# Create plot from single file, given a list of parameter names
def single_file_plot(
    folder, param_names, x_title, y_title, new_names={}, print_first_row=False
):
    # Get the parameter values
    params = get_params(folder, param_names)

    params.index = params[param_names[0]].astype(str)
    if len(param_names) > 1:
        for val in param_names[1:]:
            params.index += ", " + params[val].astype(str)

    header = list(params.index.values)

    # Get the actual results
    res = get_results(folder, header)

    if print_first_row:
        print(res.head(5))

    rpd = calc_rpd(res)

    stats = calc_stats(rpd)
    return plot_from_df(stats, x_title, y_title, new_names=new_names)


def parse_size(file):
    # get filename only
    file = file.split("/")[-1]

    # remove instance number
    file = file.split("-")[0]

    # Remove number of stages
    file = file.split("m")[0]

    # Remove 'n' and parse as int
    size = int(file.replace("n", ""))

    return size


def line_plot_from_results(df, header, x_title, y_title):
    rpd = calc_rpd(df)
    rpd["size"] = rpd.index.map(lambda x: parse_size(x))
    dfs = []

    for h in header:
        stats = rpd.groupby("size")[h + "_rpd"].agg(["mean", "sem"])
        stats["type"] = h
        dfs.append(stats)
    df = pd.concat(dfs)

    fig = go.Figure()
    for typ, stats in df.groupby("type"):
        fig.add_trace(
            go.Scatter(
                x=stats.index,
                y=stats["mean"],
                name=typ,
                error_y=dict(
                    type="data",  # value of error bar given in data coordinates
                    array=stats["sem"],
                    visible=True,
                ),
            )
        )

    fig.update_layout(
        template="simple_white",
        xaxis_type="category",
        xaxis_title=x_title,
        yaxis_title=y_title,
        font_family="Times New Roman",
    )

    fig.update_xaxes(mirror=True)
    fig.update_yaxes(mirror=True)

    return fig


def merge_and_line(folders, names, x_title, y_title):
    dfs = []
    for fname, colname in zip(folders, names):
        dfs.append(get_results(fname, [colname]))

    res = pd.concat(dfs, axis=1)

    return line_plot_from_results(res, names, x_title, y_title)


def plot_line_diagram(folder, param_names, x_title, y_title):
    params = get_params(folder, param_names)
    params.index = params[param_names[0]].astype(str)

    if len(param_names) > 1:
        for val in param_names[1:]:
            params.index += ", " + params[val].astype(str)

    header = list(params.index.values)

    res = get_results(folder, header)

    return line_plot_from_results(res, header, x_title, y_title)


if __name__ == "__main__":
    merge_and_line(
        ["../solutions/generational", "../solutions/steady_state"],
        ["Generational", "Steady"],
        "x",
        "y",
    )
