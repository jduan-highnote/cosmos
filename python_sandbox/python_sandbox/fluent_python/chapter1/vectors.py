from math import hypot


class Vector:
    def __init__(self, x=0, y=0):
        self.x = x
        self.y = y

    def __repr__(self):
        return 'Vector(%r, %r)' % (self.x, self.y)

    def __abs__(self):
        return hypot(self.x, self.y)

    def __bool__(self):
        """
        Return False if the magnitude of the vector is zero, True otherwise.
        """
        return bool(abs(self))

    def __add__(self, other):
        x = self.x + other.x
        y = self.y + other.y
        return Vector(x, y)

    def __mul__(self, scalar):
        return Vector(self.x * scalar, self.y * scalar)


def main():
    v1 = Vector(2, 4)
    v2 = Vector(2, 1)
    print('v1 + v2:', v1 + v2)

    v = Vector(3, 4)
    print('abs(v):', abs(v))

    print('v * 3:', v * 3)


if __name__ == '__main__':
    main()
