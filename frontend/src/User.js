import { Link } from "react-router-dom";

function User() {
  return (
    <div className="container">
      <Link className="link" to="/">
        Go To Main Page
      </Link>
    </div>
  );
}

export default User;
