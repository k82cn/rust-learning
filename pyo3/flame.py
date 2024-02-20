import dill
import subprocess

class Future:
    def __init__(self):
        pass

def service(fn):
    print(fn.__qualname__)

    # Connection 
    with open('service_code.pkl', 'wb') as file:
        dill.dump(fn, file)

    def service_future(*args, **kwargs):
        with open('service_args.pkl', 'wb') as file:
            dill.dump(args, file)

        subprocess.run(["./target/debug/python-shim"])

        with open('service_output.pkl', 'rb') as file:
            return dill.load(file)

    return service_future

