import { AuthV1Api, ShopV1Api } from './api'
import { authApi, setAuthApi, setShopApi } from './signals'

export default function Login() {
  let email: HTMLInputElement | undefined
  let password: HTMLInputElement | undefined

  function handleSubmit(event: Event) {
    event.preventDefault()

    authApi()
      .login(email?.value ?? '', password?.value ?? '')
      .then((reply) => {
        setAuthApi(new AuthV1Api(reply.usertoken))
        setShopApi(new ShopV1Api(reply.usertoken))
      })
      .catch((e) => {
        alert('login failed')
      })
  }

  return (
    <div>
      <h2>Login</h2>
      <form onSubmit={handleSubmit}>
        <label for="email">
          Email
          <input ref={email} name="email" />
        </label>
        <label for="password">
          Password
          <input ref={password} name="password" type="password" />
        </label>
        <button type="submit">Login</button>
      </form>
    </div>
  )
}
