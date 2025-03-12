document.querySelectorAll("input[type='checkbox'][name='role']").forEach(function(checkbox) {
    checkbox.addEventListener('click', function() {
        var box = this;

        if (box.checked) {
            var group = document.querySelectorAll("input[type='checkbox'][name='" + box.name + "']");

            group.forEach(function(otherBox) {
                otherBox.checked = false;
            });

            box.checked = true;
        } else {
            box.checked = false;
        }
    });
});

document.addEventListener('DOMContentLoaded', function () {
    const signInButton = document.getElementById('signInButton');

    signInButton.addEventListener('click', function() {
        try {
            if (!checkCheckboxes()) {
                throw new Error("Login Failed!");
            }

            if (!checkInputFields()) {
                throw new Error("Login Failed!")
            } 

            window.location.href = '../calendarHome/calendarHome.html';
        } catch(e) {
            console.log("Caught: ", e);
        }
    });
});

function checkCheckboxes() {
    let selectedRole = getCheckedValue();
    try {
        if (selectedRole == null) {
            clearErrors();
            document.getElementById("checkboxErrorDisplay").innerHTML = "You must select a role to sign in.";
            document.getElementById("checkboxError").style.display = "inline-block";
            throw new Error("No Checkbox Selected!");
        }
        return true;
    } catch(e) {
        console.log("Caught: ", e);
        return false;
    }
}

function getCheckedValue() {
    const checkboxes = document.querySelectorAll("input[type='checkbox'][name='role']");
    let selectedValue = null;

    for (let checkbox of checkboxes) {
        if (checkbox.checked) {
            selectedValue = checkbox.value;
            return selectedValue;
        }
    }
    
    return selectedValue;
}

function checkInputFields() {
    clearErrors();

    const username = document.getElementById('usernameInput').value;
    const password = document.getElementById('passwordInput').value;

    try {
        if (username == '') {
            setUsernameError("Please enter a username.");
            throw new Error("No Username Input!");
        } else {
            clearErrors();
        }

        if (password == '') {
            setPasswordError("Please enter a password.");
            throw new Error("No Password Input!");
        } else {
            clearErrors();
        }

        if (username.includes(' ')) {
            setUsernameError("Username must not contain any spaces.");
            throw new Error("Invalid Username");
        }
    
        if (password.includes(' ')) {
            setPasswordError("Password must not contain any spaces.");
            throw new Error("Invalid Password");
        }
        
        if (username.length < 3) {
            setUsernameError("Username must be at least 3 characters.");
            throw new Error("Invalid Username");
        }

        if (password.length < 8) {
            setPasswordError("Password must be at least 8 characters.");
            throw new Error("Invalid Password");
        }

        return true;
    } catch(e) {
        console.log("Caught: ", e);
        return false;
    }
}

function setUsernameError(errorString) {
    usernameFormError = document.getElementById('usernameErrorDisplay');
    usernameError = document.getElementById('usernameError');
    
    usernameFormError.innerHTML = errorString;
    usernameError.style.display = "inline-block";
}

function setPasswordError(errorString) {
    passwordFormError = document.getElementById('passwordErrorDisplay');
    passwordError = document.getElementById('passwordError');

    passwordFormError.innerHTML = errorString;
    passwordError.style.display = "inline-block";
}

function clearErrors() {
    document.getElementById('passwordError').style.display = "none";
    document.getElementById('usernameError').style.display = "none";
    document.getElementById('checkboxError').style.display = "none";
}