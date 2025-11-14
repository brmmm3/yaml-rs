import os
import time
import concurrent.futures
from typing import Tuple

import yaml
from yaml import CLoader, CDumper

import yaml_rs


def benchmark_yaml_rs(
    pathname: str, outpathname: str
) -> Tuple[float, float, float, float]:
    t1 = time.time()
    data_yaml_rs = yaml_rs.load(pathname)
    dt_load = time.time() - t1

    t1 = time.time()
    yaml_rs.save(outpathname, data_yaml_rs)
    dt_save = time.time() - t1
    t1 = time.time()

    # data_yaml_rs_check = yaml_rs.load(outpathname_yaml_rs)
    # if data_yaml_rs != data_yaml_rs_check:
    #    raise Exception("Out data of yaml_rs differs!")

    return dt_load, dt_save


def benchmark_yaml(
    pathname: str, outpathname: str
) -> Tuple[float, float, float, float]:
    t1 = time.time()
    with open(pathname, "r") as F:
        data_yaml = list(yaml.load_all(F, Loader=CLoader))
    dt_load = time.time() - t1

    t1 = time.time()
    with open(outpathname, "w") as F:
        yaml.dump_all(data_yaml, F, Dumper=CDumper)
    dt_save = time.time() - t1

    # data_yaml_check = yaml_rs.load(outpathname_yaml_rs)
    # if data_yaml != data_yaml_check:
    #    raise Exception("Out data of yaml differs!")

    return dt_load, dt_save


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


def benchmark(filename: str, iterations: int = 1):
    pathname = f"testdata/{filename}"
    outpathname = pathname.replace("testdata", "testresults")
    outpathname_yaml_rs = outpathname.replace(".yaml", "_yaml_rs.yaml")
    outpathname_yaml = outpathname.replace(".yaml", "_yaml.yaml")
    dt_yaml_rs_load_sum = 0
    dt_yaml_rs_save_sum = 0
    dt_yaml_load_sum = 0
    dt_yaml_save_sum = 0
    for _ in range(iterations):
        dt_yaml_rs_load, dt_yaml_rs_save = benchmark_yaml_rs(
            pathname, outpathname_yaml_rs
        )
        dt_yaml_load, dt_yaml_save = benchmark_yaml(pathname, outpathname_yaml)
        dt_yaml_rs_load_sum += dt_yaml_rs_load
        dt_yaml_rs_save_sum += dt_yaml_rs_save
        dt_yaml_load_sum += dt_yaml_load
        dt_yaml_save_sum += dt_yaml_save
    dt_yaml_rs_load_sum /= iterations
    dt_yaml_rs_save_sum /= iterations
    dt_yaml_load_sum /= iterations
    dt_yaml_save_sum /= iterations
    show_benchmarks(
        filename, dt_yaml_rs_load, dt_yaml_rs_save, dt_yaml_load, dt_yaml_save
    )


def benchmark_threaded(filename: str):
    pathname = f"testdata/{filename}"
    outpathname = pathname.replace("testdata", "testresults")
    pathnames = {
        outpathname.replace(".yaml", f"{i}.yaml"): (
            outpathname.replace(".yaml", f"{i}_yaml_rs.yaml"),
            outpathname.replace(".yaml", f"{i}_yaml.yaml"),
        )
        for i in range(10)
    }

    data = open(pathname, "rb").read()
    for paths in pathnames:
        with open(paths, "wb") as F:
            F.write(data)

    print("Threaded benchmark yaml_rs...")
    t1 = time.time()
    futures = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        for pathname, (outpathname_yaml_rs, outpathname_yaml) in pathnames.items():
            futures[pathname] = executor.submit(
                benchmark_yaml_rs, pathname, outpathname_yaml_rs
            )
        for version, future in futures.items():
            dt_load, dt_save = future.result()
            print(f"load={dt_load:.4f}s\tsave={dt_save:.4f}s")
    dt_yaml_rs = time.time() - t1

    print("Threaded benchmark yaml...")
    t1 = time.time()
    futures = {}
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as executor:
        for pathname, (outpathname_yaml_rs, outpathname_yaml) in pathnames.items():
            futures[pathname] = executor.submit(
                benchmark_yaml, pathname, outpathname_yaml
            )
        for pathname, future in futures.items():
            dt_load, dt_save = future.result()
            print(f"load={dt_load:.4f}s\tsave={dt_save:.4f}s")
    dt_yaml = time.time() - t1

    print(f"yaml_rs={dt_yaml_rs:.4f}s")
    print(f"yaml={dt_yaml:.4f}s")


if __name__ == "__main__":
    if not os.path.exists("testresults"):
        os.mkdir("testresults")
    benchmark("garden.yaml", 10)
    benchmark("deep.yaml", 10)
    benchmark("deeper.yaml", 10)
    benchmark("1mb.yaml")
    benchmark("5mb.yaml")
    benchmark("15mb.yaml")
    benchmark_threaded("15mb.yaml")
