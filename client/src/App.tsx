import { Show } from 'solid-js'
import { authApi, setAuthApi, setShopApi, shopApi } from './signals'
import Login from './Login'
import PetList from './PetList'
import { AuthV1Api } from './api'

export default function App() {
  function logout() {
    authApi()
      .logout()
      .then(() => {
        setAuthApi(new AuthV1Api())
        setShopApi()
      })
  }

  return (
    <div class="container">
      <nav>
        <ul>
          <li>
            <strong>Petshop</strong>
          </li>
        </ul>
        <ul>
          <Show when={shopApi()}>
            <li>
              <a href="#" onclick={logout}>
                Logout
              </a>
            </li>
          </Show>
        </ul>
      </nav>
      <Show when={shopApi()} fallback={<Login />}>
        <PetList />
      </Show>
    </div>
  )
}
