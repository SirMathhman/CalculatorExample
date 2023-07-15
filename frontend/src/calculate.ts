import axios from 'axios';

const axiosInstance = axios.create({
    baseURL: 'http://localhost:8000', // replace with your server's address
});

export const clear = async (): Promise<void> => {
    await axiosInstance.post('/clear');
};

export const addDigit = async (digit: number): Promise<void> => {
    await axiosInstance.post(`/add_digit/${digit}`);
};

export const setOperation = async (operation: string): Promise<void> => {
    const response = await axiosInstance.post(`/operation/${operation}`);
    if (response.data !== 'Operation set') {
        throw new Error(response.data);
    }
};

export const calculate = async (): Promise<number> => {
    const response = await axiosInstance.get('/calculate');
    return response.data;
};
