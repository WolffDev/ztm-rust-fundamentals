#[derive(Debug)]
struct Employee<State> {
    name: String,
    state: State,
}

impl<State> Employee<State> {
    fn transition<NextState>(self, next_state: NextState) -> Employee<NextState> {
        Employee {
            name: self.name,
            state: next_state,
        }
    }
}

#[derive(Debug)]
struct Agreement;
#[derive(Debug)]
struct Signature;
#[derive(Debug)]
struct Training;
#[derive(Debug)]
struct FailedTraining {
    score: u8,
}
#[derive(Debug)]
struct OnBoardingComplete {
    score: u8,
}

impl Employee<Agreement> {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            state: Agreement,
        }
    }
    fn read_agreement(self) -> Employee<Signature> {
        self.transition(Signature)
    }
}

impl Employee<Signature> {
    fn sign_agreement(self) -> Employee<Training> {
        self.transition(Training)
    }
}

impl Employee<Training> {
    fn train(self, score: u8) -> Result<Employee<OnBoardingComplete>, Employee<FailedTraining>> {
        if score < 7 {
            Err(self.transition(FailedTraining { score }))
        } else {
            Ok(self.transition(OnBoardingComplete { score }))
        }
    }
}

fn main() {
    let employee = Employee::new("John");
    println!("{:?}", employee);

    let onboarding = employee.read_agreement().sign_agreement().train(7);
    match onboarding {
        Ok(employee) => println!("Training Complete"),
        Err(employee) => println!("Training FAILED, score is {}", employee.state.score),
    }
}
