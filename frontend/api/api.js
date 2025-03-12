const url = "http://localhost:8787";

export async function sendRequest({ apiEndpoint, inputMethod, inputBody = null }) {
    try {
        const response = await fetch(apiEndpoint, {
            method: inputMethod,
            headers: {
                "Content-Type": "application/json",
            },
            body: inputBody ? JSON.stringify(inputBody) : null,
        });

        const responseText = await response.text();

        if (responseText === "") {
            return true;
        }

        if (response.ok) {
            let responseData = null;
            try {
                responseData = JSON.parse(responseText);
            } catch (error) {
                console.error("Error parsing JSON:", error);
                return false;
            }

            if (responseData && responseData.entries && Array.isArray(responseData.entries)) {
                return responseData.entries;
            } else {
                return true;
            }
        } else {
            return false;
        }
    } catch (error) {
        console.error('Error in sendRequest:', error);
        return false;
    }
}

export const addClass = async (classData) => {
    const apiEndpoint = url + '/class';
    const inputMethod = 'POST';
    
    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: classData,
    });

    return isSuccess;
};

export const addClassScheduleRoom = async (classData) => {
    const apiEndpoint = url + '/csr';
    const inputMethod = 'POST';
    
    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: classData,
    });

    return isSuccess;
};

export const addPreference = async (preferenceData) => {
    const apiEndpoint = url + '/pref';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: preferenceData,
    });

    return isSuccess;
};


export const getRooms = async () => {
    const apiEndpoint = url + `/room?scope=all`;
    const inputMethod = 'GET';

    try {
        const response = await sendRequest({
            apiEndpoint,
            inputMethod,
        });

        if (response && Array.isArray(response)) {
            return response;
        } else {
            return [];
        }
    } catch (error) {
        console.error("Error fetching rooms:", error);
        throw error;
    }
};

export const getFaculty = async () => {
    const apiEndpoint = url + `/faculty?scope=all`;
    const inputMethod = 'GET';

    try {
        const response = await sendRequest({
            apiEndpoint,
            inputMethod,
        });
        
        if (response && Array.isArray(response)) {
            return response;
        } else {
            return [];
        }
    } catch (error) {
        console.error("Error fetching faculty:", error);
        throw error;
    }
};


export const getSchedules = async () => {
    const apiEndpoint = url + `/schedule?scope=all`;
    const inputMethod = 'GET';

    try {
        const response = await sendRequest({
            apiEndpoint,
            inputMethod,
        });

        if (response && Array.isArray(response)) {
            return response; 
        } else {
            return [];
        }
    } catch (error) {
        console.error("Error fetching schedules:", error);
        throw error;
    }
};

export const getFeatures = async () => {
    const apiEndpoint = url + `/feature?scope=all`;
    const inputMethod = 'GET';

    try {
        const response = await sendRequest({
            apiEndpoint,
            inputMethod,
        });
        
        if (response && Array.isArray(response)) {
            return response;
        } else {
            return [];
        }
    } catch (error) {
        console.error("Error fetching features:", error);
        throw error;
    }
};

export const addRoomFeature = async (roomId, featureId) => {
    const roomFeatureData = {
        room: roomId,
        feature: parseInt(featureId)
    };

    const apiEndpoint = url + '/rf';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: roomFeatureData,
    });

    return isSuccess;
};

export const addRoom = async (roomData) => {
    const apiEndpoint = url + '/room';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: roomData,
    });

    return isSuccess;
};

export const addFaculty = async (facultyData) => {
    const apiEndpoint = url + '/faculty';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: facultyData,
    });

    return isSuccess;
};

export const addSchedule = async (scheduleData) => {
    const apiEndpoint = url + '/schedule';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: scheduleData
    });

    return isSuccess;
};

export const addFeature = async (scheduleData) => {
    const apiEndpoint = url + '/feature';
    const inputMethod = 'POST';

    const isSuccess = await sendRequest({
        apiEndpoint,
        inputMethod,
        inputBody: scheduleData
    });

    return isSuccess;
};