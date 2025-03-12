import { addRoom, getFeatures, addRoomFeature, getRooms } from '../api/api.js';

document.getElementById('submitRoomButton').addEventListener('click', function(event) {
    event.preventDefault();

    try {
        if (!testRoomFields() || !checkRoomDropdown()) {
            throw new Error("Room Form Error");
        } else {
            clearErrors();
        }

        const number = document.getElementById("roomNumber").value;
        const capacity = document.getElementById("capacity").value;
        const kind = document.getElementById("roomType").value;

        if (!number || !capacity || !kind) {
            throw new Error("Room creation error.");
        }

        const roomData = {
            number,
            capacity: parseInt(capacity),
            kind,
        };

        addRoom(roomData)
            .then((isSuccess) => {
                const notificationBanner = document.getElementById("notificationBanner");

                if (isSuccess) {
                    updateNotificationBanner("Room has been successfully added!", true);

                    const selectedFeatureIds = Array.from(document.querySelectorAll('input[name="classFeatures[]"]:checked'))
                        .map(checkbox => checkbox.id);

                    addRoomFeaturesToRoom(selectedFeatureIds);

                    document.getElementById("roomNumber").value = '';
                    document.getElementById("capacity").value = '';
                    document.getElementById("roomType").value = '';
                } else {
                    updateNotificationBanner("Error: Room was not added!", false);
                    throw new Error("Room creation error.");
                }
            })
            .catch((e) => {
                updateNotificationBanner("An error occurred: " + e, false);
                throw new Error("Room creation error.");
            });
    } catch (e) {
        console.log("Caught error: ", e);
    }
});

const getMostRecentRoomId = async () => {
    const rooms = await getRooms();
    if (rooms && rooms.length > 0) {
        const mostRecentRoom = rooms[rooms.length - 1];
        return mostRecentRoom.id;
    }
    return null;
};

export const addRoomFeaturesToRoom = async (selectedFeatureIds) => {
    try {
        const roomId = await getMostRecentRoomId();

        if (!roomId) {
            throw new Error("No room found to associate features with.");
        }

        for (const featureId of selectedFeatureIds) {
            const success = await addRoomFeature(roomId, featureId);
            if (!success) {
                throw new Error(`Failed to add feature ${featureId} to room ${roomId}.`);
            }
        }

        console.log("All selected features have been added to the room.");
    } catch (error) {
        console.error("Error adding room features:", error);
    }
};

const populateRoomFeatures = async () => {
    const featuresContainer = document.getElementById("featuresContainer");
    const classFeatureError = document.getElementById("featureError");
    const classFeatureErrorDisplay = document.getElementById("featureErrorDisplay");

    try {
        const features = await getFeatures();

        if (features && features.length > 0) {
            for (let i = 0; i < features.length; i++) {
                const feature = features[i];

                const checkboxContainer = document.createElement("div");
                checkboxContainer.classList.add("form-check");

                const checkbox = document.createElement("input");
                checkbox.type = "checkbox";
                checkbox.classList.add("form-check-input");
                checkbox.id = feature.id; // Feature ID
                checkbox.name = "classFeatures[]";
                checkbox.value = feature.data.name;

                const label = document.createElement("label");
                label.classList.add("form-check-label");
                label.setAttribute("for", feature.id);
                label.textContent = feature.data.name;

                checkboxContainer.appendChild(checkbox);
                checkboxContainer.appendChild(label);

                featuresContainer.appendChild(checkboxContainer);
            }

            classFeatureError.style.display = 'none';
        } else {
            const errorMessage = "No features available.";
            classFeatureErrorDisplay.textContent = errorMessage;
            classFeatureError.style.display = 'inline-block';
        }
    } catch (error) {
        console.error("Failed to populate room features:", error);

        classFeatureErrorDisplay.textContent = "Error loading features.";
        classFeatureError.style.display = 'inline-block';
    }
};

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

function testRoomFields() {
    const roomNumber = document.getElementById("roomNumber");
    const capacity = document.getElementById("capacity");

    if (roomNumber.value.trim() === "") {
        setErrorText("roomNumberErrorDisplay", "roomNumberError", "Room number cannot be empty.");
        return false;
    } else {
        clearError("roomNumberError");
    }

    if (capacity.value <= 0) {
        setErrorText("roomCapacityErrorDisplay", "roomCapacityError", "Room capacity must be a number, and cannot be empty or less than 1.");
        return false;
    } else {
        clearError("roomCapacityError");
    }

    return true;
}

function checkRoomDropdown() {
    const roomType = document.getElementById("roomType");

    if (roomType.value === "") {
        setErrorText("roomTypeErrorDisplay", "roomTypeError", "Please select a room type.");
        return false;
    } else {
        clearError("roomTypeError");
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
    const errorElements = ['roomNumberError', 'roomCapacityError', 'roomTypeError'];
    errorElements.forEach(id => {
        clearError(id);
    });
}

document.addEventListener('DOMContentLoaded', function() {
    populateRoomFeatures();
}); 