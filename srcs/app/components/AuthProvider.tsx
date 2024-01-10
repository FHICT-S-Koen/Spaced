import axios from 'axios';
import {
  type JSXElement,
  createSignal,
  useContext,
  createContext,
  Show,
} from 'solid-js';

const [isOpen, setIsOpen] = createSignal(true);
const [isRegistration, setIsRegistration] = createSignal(true);

function register(event: SubmitEvent) {
  event.preventDefault();
  const formData = new FormData(event.target);
  const email = formData.get('email');
  const username = formData.get('username');
  const password = formData.get('password');
  axios
    .post('/api/user/register', {
      email,
      username,
      password,
    })
    .then((payload) => {
      setIsOpen(false);
      localStorage.setItem('access_token', payload.data.access_token);
    })
    .catch(() => {
      localStorage.removeItem('access_token');
    });
}

function login(event: SubmitEvent) {
  event.preventDefault();
  const formData = new FormData(event.target);
  const email = formData.get('email');
  const password = formData.get('password');
  axios
    .post('/api/user/login', {
      email,
      password,
    })
    .then((payload) => {
      setIsOpen(false);
      localStorage.setItem('access_token', payload.data.access_token);
    })
    .catch(() => {
      localStorage.removeItem('access_token');
    });
}

const context = { register };
const AuthContext = createContext(context);

type AuthProps = {
  children: JSXElement;
};

export function AuthProvider(props: AuthProps) {
  return (
    // eslint-disable-next-line @typescript-eslint/ban-ts-comment
    // @ts-ignore Ignore since getters and setters are already present
    <AuthContext.Provider>
      <Show when={isOpen()}>
        <dialog
          open={isOpen()}
          class="z-[9999] flex h-full w-full justify-center rounded bg-transparent p-2 align-middle text-white backdrop-blur-[1px]"
        >
          <form
            onSubmit={(e) => (isRegistration() ? register(e) : login(e))}
            class="w-1/5"
          >
            <div class="h-44">
              <label class="flex justify-between">
                Email:
                <input
                  type="email"
                  name="email"
                  class="rounded p-1 text-black"
                  placeholder="user@example.com"
                />
              </label>
              <br />
              <Show when={isRegistration()}>
                <label class="flex justify-between">
                  Username:
                  <input
                    type="text"
                    name="username"
                    class="rounded p-1 text-black"
                  />
                </label>
                <br />
              </Show>
              <label class="flex justify-between">
                Password:
                <input
                  type="password"
                  name="password"
                  class="rounded p-1 text-black"
                />
              </label>
            </div>
            <br />
            <button
              class="float-right w-32 rounded bg-slate-400 p-1"
              type="submit"
            >
              Submit
            </button>
            {isRegistration() ? (
              <button
                class="w-48 rounded bg-slate-400 p-1"
                onClick={() => setIsRegistration(false)}
              >
                Login
              </button>
            ) : (
              <button
                class="w-48 rounded bg-slate-400 p-1"
                onClick={() => setIsRegistration(true)}
              >
                Register a new account
              </button>
            )}
          </form>
        </dialog>
      </Show>
      {props.children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  return useContext(AuthContext);
}
