import { addFeature } from '../api/api.js';

document.getElementById('submitFeatureButton').addEventListener('click', function(event) {
    event.preventDefault();

    try {
        if (!testFeatureFields()) {
            throw new Error("Feature Form Error");
        } else {
            clearErrors();
        }

        const name = document.getElementById("featureName").value;
        const description = document.getElementById("featureDescription").value;

        if (!name || !description) {
            throw new Error("Feature creation error.");
        }

        const featureData = {
            name,
            description,
        };

        addFeature(featureData)
            .then((isSuccess) => {
                const notificationBanner = document.getElementById("notificationBanner");

                if (isSuccess) {
                    updateNotificationBanner("Feature has been successfully added!", true);

                    document.getElementById("featureName").value = '';
                    document.getElementById("featureDescription").value = '';
                } else {
                    updateNotificationBanner("Error: Feature was not added!", false);
                    throw new Error("Feature creation error.");
                }
            })
            .catch((e) => {
                updateNotificationBanner("An error occurred: " + e, false);
                throw new Error("Feature creation error.");
            });
    } catch (e) {
        console.log("Caught error: ", e);
    }
});

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

function testFeatureFields() {
    const featureName = document.getElementById("featureName");
    const featureDescription = document.getElementById("featureDescription");

    if (featureName.value.trim() === "") {
        setErrorText("featureNameErrorDisplay", "featureNameError", "Feature name cannot be empty.");
        return false;
    } else {
        clearError("featureNameError");
    }

    if (featureDescription.value.trim() === "") {
        setErrorText("featureDescriptionErrorDisplay", "featureDescriptionError", "Feature description cannot be empty.");
        return false;
    } else {
        clearError("featureDescriptionError");
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
    const errorElements = ['featureNameError', 'featureDescriptionError'];
    errorElements.forEach(id => {
        clearError(id);
    });
}