import axios from 'axios';
import {
  type JSXElement,
  createSignal,
  useContext,
  createContext,
  Show,
} from 'solid-js';

const [isLoggedIn, setIsLoggedIn] = createSignal(false);
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
      setIsLoggedIn(true);
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
      setIsLoggedIn(true);
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
      <Show when={!isLoggedIn()}>
        <dialog
          open={!isLoggedIn()}
          class="inset-0 z-[9999] flex h-[460px] items-center justify-center rounded"
        >
          <form
            onSubmit={(e) => (isRegistration() ? register(e) : login(e))}
            class="flex h-full w-96 flex-col overflow-hidden rounded-md p-8 shadow-md"
          >
            <h2 class="mb-4 text-center text-2xl font-bold text-gray-800">
              {isRegistration() ? 'Register' : 'Login to account'}
            </h2>
            <div class="mb-6 flex-grow">
              <label class="mb-2 block font-semibold text-gray-800">
                Email
                <input
                  type="email"
                  name="email"
                  class="w-full rounded border px-3 py-2 outline-none transition-all duration-300 focus:border-blue-500"
                  placeholder="user@example.com"
                />
              </label>
              <Show when={isRegistration()}>
                <label class="mb-2 block font-semibold text-gray-800">
                  Username
                  <input
                    type="text"
                    name="username"
                    class="w-full rounded border px-3 py-2 outline-none transition-all duration-300 focus:border-blue-500"
                    placeholder="username"
                  />
                </label>
              </Show>
              <label class="mb-2 block font-semibold text-gray-800">
                Password
                <input
                  type="password"
                  name="password"
                  class="w-full rounded border px-3 py-2 outline-none transition-all duration-300 focus:border-blue-500"
                  placeholder="•••••••••••••"
                />
              </label>
            </div>
            <button
              class="w-full rounded-md bg-blue-500 px-4 py-2 text-white outline-none transition-all duration-300 hover:bg-blue-600"
              type="submit"
            >
              Submit
            </button>
            <div class="mt-6 text-center text-gray-800">
              {isRegistration() ? (
                <p>
                  Already have an account?{' '}
                  <button
                    class="text-blue-500 transition-all duration-300 hover:underline focus:outline-none"
                    onClick={() => setIsRegistration(false)}
                  >
                    Login here
                  </button>
                </p>
              ) : (
                <p>
                  Don't have an account?{' '}
                  <button
                    class="text-blue-500 transition-all duration-300 hover:underline focus:outline-none"
                    onClick={() => setIsRegistration(true)}
                  >
                    Register here
                  </button>
                </p>
              )}
            </div>
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
