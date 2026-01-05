def carFleet(target, position, speed):
    cars = sorted(zip(position, speed), reverse=True)
    fleets = 0
    max_time = 0
    for pos, spd in cars:
        time = (target - pos) / spd
        if time > max_time:
            fleets += 1
            max_time = time
    return fleets
