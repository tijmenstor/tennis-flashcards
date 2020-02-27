import React from 'react';
import logo from './assets/logo.svg';
import './App.css';
import { Navbar, Container, Row, Col, Card, Button, Fade } from 'react-bootstrap';

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      fadeIn: false
    }
  }

  componentDidMount() {
    this.interval = setInterval(() => this.setState({ fadeIn: true }), 500);
  }

  render() {
    return (
      <div className="App">
        <div className="bg"></div>
        <Container fluid className="base-container">
          <Row style={{ display: 'block' }}>
            <Navbar bg="dark" variant="dark">
              <Navbar.Brand href="/">
                <img
                  src={logo}
                  width="29"
                  height="29"
                  className="d-inline-block align-top"
                  alt="Logo"
                />{' '}
                Tennis Questions
              </Navbar.Brand>
            </Navbar>
          </Row>
          <Row className="align-items-md-center" style={{ height: '100%' }}>
            <Fade in={this.state.fadeIn}>
              <Col className="text-left align-self-start" md={3} style={{top: '30px'}}>
                <Col md={9}>
                  <h4>Current Highscores</h4>
                  <hr className="white-line" />
                  <ol>
                    <li>Peter</li>
                    <li>Henk</li>
                    <li>Gerrit</li>
                  </ol>
                </Col>
              </Col>
            </Fade>
            <Col md={6}>
                <Fade in={this.state.fadeIn}>
                  <Card className="card-background-transparent">
                    <Card.Body>
                      <Card.Title className="card-title-text">Tennis Questions</Card.Title>
                      <Card.Subtitle>A competitive tennis quiz!</Card.Subtitle>
                      <hr className="white-line" />
                      <Card.Text>
                        Tennis Questions is a small website where you can start a quiz based on tennis!<br />
                        When you start, you will get 10 different questions, where every question can be of three kinds:<br /><br />
                        <ul>
                          <li>Short answer: "Roger Federer", "US-Open"</li>
                          <li>Multiple-choice: Nadal, Edberg, Chang or Agassi</li>
                          <li>True or false: Did Djokovic win the Australian Open 2020?</li>
                        </ul>
                        At the end, you can see how you did and enter your name. Is your score high enough?
                        Then you might find yourself on the highscores! &#128521;
                      </Card.Text>
                      <Button variant="dark">Clickety-click!</Button>
                    </Card.Body>
                  </Card>
                </Fade>
              </Col>
          </Row>
        </Container>
      </div>
    )
  }
}
export default App;
