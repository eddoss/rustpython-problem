import problem

foo = problem.Foo(
    items=["one", "two"]
)
foo.items.append("free")    # this does not work
print(foo.items)            # expect: ['one', 'two', 'free']
                            # actual: ['one', 'two']