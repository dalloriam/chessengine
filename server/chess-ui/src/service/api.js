import axios from 'axios';

const HOST = "http://localhost:3030";

export class API {
    async getPosition() {
        let response = await axios.get(`${HOST}/position`);
        return response.data.position_fen;
    }

    async move(s, d) {
        let response = await axios.post(`${HOST}/move`, { src: s, dst: d });
        console.log(response);
        return {
            position: response.data.position_fen,
            error: response.data.error
        }
    }
}
