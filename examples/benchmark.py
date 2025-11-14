import os
import time
from typing import Tuple

import yaml

import yaml_rs


def benchmark_do(filename: str) -> Tuple[float, float, float, float]:
    pathname = f"testdata/{filename}"
    outpathname = pathname.replace("testdata", "testresults")
    outpathname_yaml_rs = outpathname.replace(".yaml", "_yaml_rs.yaml")
    outpathname_yaml = outpathname.replace(".yaml", "_yaml.yaml")

    if not os.path.exists("testresults"):
        os.mkdir("testresults")

    t1 = time.time()
    data_yaml_rs = yaml_rs.load(pathname)
    dt_yaml_rs_load = time.time() - t1

    t1 = time.time()
    yaml_rs.save(outpathname_yaml_rs, data_yaml_rs[0])
    dt_yaml_rs_save = time.time() - t1

    # data_yaml_rs_check = yaml_rs.load(outpathname_yaml_rs)
    # if data_yaml_rs != data_yaml_rs_check:
    #    raise Exception("Out data of yaml_rs differs!")

    t1 = time.time()
    with open(pathname, "r") as F:
        data_yaml = list(yaml.safe_load_all(F))
    dt_yaml_load = time.time() - t1

    t1 = time.time()
    with open(outpathname_yaml, "w") as F:
        yaml.dump_all(data_yaml[0], F)
    dt_yaml_save = time.time() - t1

    # data_yaml_check = yaml_rs.load(outpathname_yaml_rs)
    # if data_yaml != data_yaml_check:
    #    raise Exception("Out data of yaml differs!")

    return dt_yaml_rs_load, dt_yaml_rs_save, dt_yaml_load, dt_yaml_save


def show_benchmarks(
    filename: str,
    dt_yaml_rs_load: float,
    dt_yaml_rs_save: float,
    dt_yaml_load: float,
    dt_yaml_save: float,
):
    print(f"Filename: {filename}")
    print(f"yaml_rs load: {dt_yaml_rs_load:.4f}s")
    print(f"yaml_rs save: {dt_yaml_rs_save:.4f}s")
    print(f"yaml load: {dt_yaml_load:.4f}s")
    print(f"yaml save: {dt_yaml_save:.4f}s")
    print(f"load performance: {dt_yaml_load / dt_yaml_rs_load:.2f} x faster")
    print(f"save performance: {dt_yaml_save / dt_yaml_rs_save:.2f} x faster")
    print()


def benchmark(filename: str):
    dt_yaml_rs_load, dt_yaml_rs_save, dt_yaml_load, dt_yaml_save = benchmark_do(
        filename
    )
    show_benchmarks(
        filename, dt_yaml_rs_load, dt_yaml_rs_save, dt_yaml_load, dt_yaml_save
    )


benchmark("garden.yaml")
benchmark("deep.yaml")
benchmark("deeper.yaml")
benchmark("15mb.yaml")
