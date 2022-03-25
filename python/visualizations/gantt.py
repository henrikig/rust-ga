import json
import pandas as pd
import plotly.express as px
from datetime import datetime, date, time, timedelta
import re
import sys


FILE = "../../solutions/ga/n20m2-01.json"
INSTANCE = "../../instances/ruiz/json/n20m2-01.json"


def visualize_schedule(solution: str, instance: str):
    with open(solution, "r") as f:
        solution: str = json.load(f)

    with open(instance, "r") as f:
        instance: str = json.load(f)

    schedule: list = []

    production_start = datetime.combine(date.today(), time()) + timedelta(hours=8)

    for stage in range(solution["stages"]):
        for machine in range(solution["machines"][stage]):
            for machine_run, (job, completion_time) in enumerate(
                solution["machine_completions"][stage][machine]
            ):
                # get current job's processing time
                production_time = instance["processing_times"][job][stage]

                # get current job's setup time
                if machine_run == 0:
                    setup_time = instance["setup_times"][stage][job][job]
                else:
                    prev_job, _ = solution["machine_completions"][stage][machine][
                        machine_run - 1
                    ]
                    setup_time = instance["setup_times"][stage][prev_job][job]

                end_time = production_start + timedelta(minutes=completion_time)
                # Start time is completion time - (processing time + setup time)
                start_time = production_start + timedelta(
                    minutes=completion_time - production_time - setup_time
                )
                schedule.append(
                    dict(
                        Machine=f"{stage + 1, machine + 1}",
                        Start=start_time,
                        End=end_time,
                        Job=f"J{job+1}",
                    )
                )

    df = pd.DataFrame(schedule)

    legend_order = [f"J{job}" for job in range(instance["jobs"])]
    legend_order.sort(key=natural_keys)

    fig = px.timeline(
        df,
        x_start="Start",
        x_end="End",
        y="Machine",
        color="Job",
        category_orders={"Job": legend_order},
    )

    # Order machines with the lowest stages at the top
    fig.update_yaxes(categoryorder="category descending")

    # Set font family to Times New Roman
    fig.update_layout(font_family="Times New Roman")
    fig.show()


def atoi(text):
    return int(text) if text.isdigit() else text


def natural_keys(text):
    """
    alist.sort(key=natural_keys) sorts in human order
    http://nedbatchelder.com/blog/200712/human_sorting.html
    (See Toothy's implementation in the comments)
    """
    return [atoi(c) for c in re.split(r"(\d+)", text)]


if __name__ == "__main__":
    import os

    print(os.getcwd())

    if len(sys.argv) > 1:
        instance = sys.argv[1]
        FILE = "../solutions/ga/" + instance
        INSTANCE = "../instances/ruiz/json/" + instance

    visualize_schedule(FILE, INSTANCE)
