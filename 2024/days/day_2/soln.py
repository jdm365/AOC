def safe_checker(report: list[int], dampener_level: int = 0) -> bool:
    current_diff = report[1] - report[0]
    for i, _ in enumerate(report[:-1]):
        next_diff = report[i+1] - report[i]
        if next_diff == 0 or abs(next_diff) > 3 or (next_diff > 0) != (current_diff > 0):
            if not dampener_level:
                return (safe_checker(report[:i]+report[i+1:], dampener_level + 1) or
                    safe_checker(report[:i-1]+report[i:], dampener_level + 1) or
                    safe_checker(report[:i+1]+report[i+2:], dampener_level + 1))
            else:
                return False
        current_diff = next_diff
    return True


if __name__ == "__main__":
    with open("input.txt", "r") as f:
        data = [[int(x) for x in line.split()] for line in f]

    safe_count = sum([safe_checker(line, 1) for line in data])
    safe_count_damping = sum([safe_checker(line) for line in data])

    print(f"Part 1: {safe_count}")
    print(f"Part 2: {safe_count_damping}")

    print([int(safe_checker(line)) for line in data])
