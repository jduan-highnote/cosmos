import json
import operator
import re
import redis
import time
from itertools import (groupby, chain)


def get_builds_from_redis(num_of_days):
    """
    Get broken builds from Redis via pagination.
    :return:
    :rtype:
    """
    rds = redis.Redis(host="localhost", port=6861, db=0)
    end = now_in_millis()
    start = one_day_ago(end)
    broken_builds = []
    for x in range(num_of_days):
        items = rds.zrangebyscore("treehouse_builds", start, end)
        for item in items:
            build = json.loads(item)
            custom_values = parse_custom_values(build)
            if ('Git branch' in custom_values
                    and custom_values['Git branch'] == 'refs/heads/master'
                    and build['isCI'] == 'true'
                    and build['isSuccessful'] == 'false'):
                broken_builds.append(build)
        end = start
        start = one_day_ago(end)

    return broken_builds


def parse_custom_values(build):
    custom_values = build['customValues']
    matches = re.findall(r"\((.*?)\)", custom_values)
    ret = dict()
    for match in matches:
        key, value = match.split(", ")
        ret[key] = value
    return ret


def now_in_millis():
    return int(round(time.time() * 1000))


def one_day_ago(millis):
    # return millis - (60 * 60 * 1000)
    return millis - (24 * 60 * 60 * 1000)


def analyze_particular_task_root_cause(builds):
    def epoch_to_string(start_time):
        lt = time.localtime(int(start_time) / 1000)
        return time.strftime("%m/%d/%Y", lt)

    for build in builds:
        task_root_cause = build['taskRootCause']
        if task_root_cause and "finished with non-zero exit value 137" in task_root_cause:
            print("%s,%s,%s" % (epoch_to_string(build['startTime']), build['id'], build['failedTasks']))


# keys:
#  'id', 'startTime', 'duration', 'tasks', 'customValues', 'isSuccessful', 'isCI',
#  'failureReason', 'failedTasks', 'taskRootCause', 'failedTests', 'testFailureRootCause'
def analyze_failure_reasons(builds):
    sort_func = lambda build : build["failureReason"]
    sorted_builds = sorted(builds, key=sort_func)
    groups = groupby(sorted_builds, key=sort_func)
    print_groups(groups)


def analyze_compilation_failures(builds):
    builds = [build for build in builds if build["failureReason"] == "COMPILATION_FAILURE"]
    sort_func = lambda build : build["failedTasks"]
    sorted_builds = sorted(builds, key=sort_func)
    groups = groupby(sorted_builds, key=sort_func)
    print_groups(groups)


def analyze_other_failures(builds):
    builds = [build for build in builds if build["failureReason"] == "OTHER"]
    failed_tasks = []
    for build in builds:
        failed_tasks.append(build["failedTasks"].split(","))
    failed_tasks = list(chain(*failed_tasks))
    sort_func = lambda task : task.split(":")[-1]
    failed_tasks = sorted(failed_tasks, key=sort_func)
    groups = groupby(failed_tasks, key=sort_func)
    print_groups(groups)


def print_groups(groups):
    new_groups = dict([(key, len(list(value))) for (key, value) in groups])
    sorted_groups = sorted(new_groups.items(), key=operator.itemgetter(1), reverse=True)
    for key, value in sorted_groups:
        print("%s: %s" % (key, value))


if __name__ == "__main__":
    main()
