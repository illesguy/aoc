import os
import sys


current_dir = os.path.dirname(os.path.abspath(__file__))
input_dir = f'{current_dir}/../../inputs'


def get_input_for_day(year, day, parse_input=lambda i: i, separator=None):
    download_input_if_not_exists(year, day)
    print(f'Reading input for {year} day {day}')
    with open(f'{input_dir}/{year}/day{day}.txt', 'r') as fl:
        if separator is not None:
            return [parse_input(l.strip()) for l in fl.read().split(separator)]
        else:
            return [parse_input(l.strip()) for l in fl.readlines()]


def download_input_if_not_exists(year, day):
    file_to_download = f'{input_dir}/{year}/day{day}.txt'
    if os.path.exists(file_to_download):
        print(f'Input for {year} day {day} already exists locally')
        return

    print(f'Downloading input for {year} day {day}')
    session_id = os.environ.get('AOC_SESSION_ID')
    if session_id is None:
        print('Session ID is missing, set env variable AOC_SESSION_ID and try again')
        sys.exit(-1)

    import requests
    resp = requests.get(
        f'https://adventofcode.com/{year}/day/{day}/input',
        headers={'Cookie': f'session={session_id}'}
    )
    with open(file_to_download, 'w') as fl:
        fl.write(resp.text.strip())

    print(f'Input successfully downloaded for {year} day {day}')
    

if __name__ == "__main__":
    download_input_if_not_exists(sys.argv[1], sys.argv[2])
