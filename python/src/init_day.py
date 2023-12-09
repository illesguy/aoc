import argparse
from datetime import datetime
import os
from inputs import download_input_if_not_exists


def parse_args():
    today = datetime.today()
    arg_parser = argparse.ArgumentParser(description='Download input for a given day')
    arg_parser.add_argument('-y', '--year', type=int, help='Year', default=today.year)
    arg_parser.add_argument('-d', '--day', type=int, help='Day', default=today.day)
    return arg_parser.parse_args()


def create_day_file(year, day):
    day_file_name = f"{year}/day{day}.py"
    if os.path.exists(day_file_name):
        print(f"File {day_file_name} already exists")
        return
    with open(day_file_name, "w") as day_file:
        with open("day_file_template.txt") as template_file:
            day_file.write(template_file.read().format(year=year, day=day))
    print(f"Created file {day_file_name}")


def main():
    args = parse_args()
    download_input_if_not_exists(args.year, args.day)
    create_day_file(args.year, args.day)


if __name__ == "__main__":
    main()
