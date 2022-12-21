with open("input.txt") as file:
    contents = file.read()
    repetition = False
    start = 0
    stop = 4
    signal = False

    while signal is False:
        chunk = contents[start:stop:1]
        start += 1
        stop += 1
        print(chunk)
        chunk_list = [letter for letter in chunk]
        for letter in chunk:
            if chunk.count(letter) > 1:
                chunk_list.remove(letter)

        if len(chunk_list) == 4:
            print(stop - 1)
            signal = True
