import { Link } from "react-router-dom";

function App() {
  return (
    <div className="container">
      <Link className="link" to="/user">
        Go To User Page
      </Link>
    </div>
  );
}

export default App;
