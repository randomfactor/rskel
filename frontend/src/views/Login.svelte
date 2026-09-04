<script>
  function goToGoogleLogin() {
    window.location.href = '/auth/google/login'
  }
</script>

<main class="auth-shell">
  <section class="card">
    <p class="eyebrow">Access</p>
    <h1>Sign In</h1>

    <div class="provider-list">
      <button class="primary-button" onclick={goToGoogleLogin}>Continue with Google</button>
      <button class="secondary-button" disabled>Microsoft (Coming Soon)</button>
      <button class="secondary-button" disabled>Yahoo (Coming Soon)</button>
    </div>

    <a href="#/" class="back-link">← Back to Home</a>
  </section>
</main>

<style>
  .auth-shell {
    min-height: 100vh;
    display: grid;
    place-items: center;
    background: linear-gradient(180deg, #f8fafc 0%, #eef2ff 100%);
    padding: 2rem;
  }

  .card {
    width: min(100%, 460px);
    background: white;
    border: 1px solid #e5e7eb;
    border-radius: 20px;
    box-shadow: 0 24px 56px rgba(15, 23, 42, 0.08);
    padding: 2rem;
  }

  .eyebrow {
    margin: 0 0 0.5rem;
    color: #6d28d9;
    font-size: 0.8rem;
    font-weight: 700;
    letter-spacing: 0.12em;
    text-transform: uppercase;
  }

  h1 {
    margin: 0 0 1.5rem;
    font-size: clamp(2rem, 3vw, 2.5rem);
    color: #111827;
  }

  .provider-list {
    display: grid;
    gap: 0.75rem;
  }

  .primary-button,
  .secondary-button {
    width: 100%;
    border-radius: 12px;
    padding: 0.9rem 1rem;
    font-size: 1rem;
    font-weight: 700;
    border: 1px solid transparent;
    transition: transform 0.2s ease;
  }

  .primary-button {
    background: linear-gradient(135deg, #2563eb 0%, #7c3aed 100%);
    color: white;
    border: none;
    cursor: pointer;
  }

  .secondary-button {
    background: #f8fafc;
    color: #475569;
    border-color: #e2e8f0;
    cursor: not-allowed;
  }

  .primary-button:hover:not(:disabled) {
    transform: translateY(-1px);
  }

  .back-link {
    display: inline-block;
    margin-top: 1.25rem;
    color: #4f46e5;
    text-decoration: none;
    font-weight: 600;
  }
</style>
