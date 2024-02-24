class Solution:
    def compress(self, chars: List[str]) -> int:
        result = chars[0]
        counter = 1
        previous = chars[0]
        for char in chars[1:]:
            if char == previous:
                counter += 1
            else:
                if counter > 1:
                    result += str(counter)
                counter = 1
                previous = char
                result += char

        if counter > 1:
            result += str(counter)

        chars[:] = list(result)

        return len(result)

