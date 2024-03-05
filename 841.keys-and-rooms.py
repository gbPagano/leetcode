from collections import deque


class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        seen = [False] * len(rooms)
        seen[0] = True
        q = deque(rooms[0])
        while q:
            curr = q.popleft()
            if not seen[curr]:
                for key in rooms[curr]:
                    q.append(key)
                seen[curr] = True

        return all(seen)
