import { addFaculty, getFaculty, addPreference } from '../api/api.js';

const getMostRecentFacultyId = async () => {
    const faculty = await getFaculty();

    if (faculty && faculty.length > 0) {
        const mostRecentFaculty = faculty[faculty.length - 1];
        return mostRecentFaculty.id + 1; 
    }
    
    return 1; 
};

document.getElementById('submitFacultyButton').addEventListener('click', async function(event) {
    event.preventDefault();

    try {
        if (!testFacultyFields()) {
            throw new Error("Faculty Form Error");
        } else {
            clearErrors();
        }

        const name = document.getElementById("facultyName").value;
        const email = document.getElementById("facultyEmail").value;
        const department = document.getElementById("facultyDepartment").value;
        const preference = document.getElementById("facultyPreference").value;

        if (!name || !email || !department) {
            throw new Error("Faculty creation error. Missing required fields.");
        }

        const facultyId = await getMostRecentFacultyId();

        const facultyData = {
            id: facultyId,
            name,
            email,
            department,
        };

        addFaculty(facultyData)
            .then((isSuccess) => {
                const notificationBanner = document.getElementById("notificationBanner");

                if (isSuccess) {
                    updateNotificationBanner("Faculty has been successfully added!", true);

                    if (preference) {
                        const preferenceData = {
                            faculty: facultyId,
                            kind: "preference",
                            value: preference,
                        };

                        addPreference(preferenceData)
                            .then((preferenceSuccess) => {
                                if (preferenceSuccess) {
                                    updateNotificationBanner("Preference has been successfully added!", true);
                                } else {
                                    updateNotificationBanner("Error: Preference was not added!", false);
                                }
                            })
                            .catch((error) => {
                                updateNotificationBanner("Error: Preference addition failed.", false);
                            });
                    }

                    // Reset form fields
                    document.getElementById("facultyName").value = '';
                    document.getElementById("facultyEmail").value = '';
                    document.getElementById("facultyDepartment").value = '';
                    document.getElementById("facultyPreference").value = '';
                } else {
                    updateNotificationBanner("Error: Faculty was not added!", false);
                    throw new Error("Faculty creation error.");
                }
            })
            .catch((e) => {
                updateNotificationBanner("An error occurred: " + e, false);
                throw new Error("Faculty creation error.");
            });
    } catch (e) {
        console.log("Caught error: ", e);
    }
});

function testFacultyFields() {
    const facultyName = document.getElementById("facultyName");
    const facultyEmail = document.getElementById("facultyEmail");
    const facultyDepartment = document.getElementById("facultyDepartment");

    if (facultyName.value.trim() === "") {
        setErrorText("facultyNameErrorDisplay", "facultyNameError", "Faculty name cannot be empty.");
        return false;
    } else {
        clearError("facultyNameError");
    }

    if (facultyEmail.value.trim() === "") {
        setErrorText("facultyEmailErrorDisplay", "facultyEmailError", "Faculty email cannot be empty.");
        return false;
    } else {
        clearError("facultyEmailError");
    }

    if (facultyDepartment.value.trim() === "") {
        setErrorText("facultyDepartmentErrorDisplay", "facultyDepartmentError", "Faculty department cannot be empty.");
        return false;
    } else {
        clearError("facultyDepartmentError");
    }

    return true;
}

function setErrorText(inputErrorElement, inputErrorText, text) {
    const errorElement = document.getElementById(inputErrorElement);
    const errorText = document.getElementById(inputErrorText);

    errorElement.innerHTML = text;
    errorText.style.display = "inline-block";
}

function clearError(inputErrorText) {
    const errorText = document.getElementById(inputErrorText);
    errorText.style.display = "none";
}

function clearErrors() {
    const errorElements = ['facultyNameError', 'facultyEmailError', 'facultyDepartmentError'];
    errorElements.forEach(id => {
        clearError(id);
    });
}

function updateNotificationBanner(message, isSuccess) {
    const notificationBanner = document.getElementById("notificationBanner");

    notificationBanner.textContent = message;

    if (isSuccess) {
        notificationBanner.classList.remove('alert-danger');
        notificationBanner.classList.add('alert-success');
    } else {
        notificationBanner.classList.remove('alert-success');
        notificationBanner.classList.add('alert-danger');
    }

    notificationBanner.classList.remove('d-none');

    setTimeout(() => {
        notificationBanner.classList.add('d-none');
    }, 5000);
}