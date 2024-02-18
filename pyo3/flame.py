import dill

class Future:
    def __init__(self):
        pass

def service(fn):
    print(fn.__qualname__)

    # Connection 
    with open('service_code.pkl', 'wb') as file:
        dill.dump(fn, file)

    def service_future(*args, **kwargs):
        with open('service_input.pkl', 'wb') as file:
            dill.dump((args, kwargs), file)

        for k, v in kwargs.items():
            ret = fn(*args, **kwargs)

        return ret

    return service_future

